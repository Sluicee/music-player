mod audio;
mod scanner;

use audio::{create_player, PlaybackState, SharedPlayer};
use scanner::{calculate_library_size, scan_folder_streaming};
use tauri_plugin_dialog::DialogExt;
use tauri::{Manager, Emitter};

// ── Dialog command ────────────────────────────────────────────────────────────

#[tauri::command]
fn pick_folder(app: tauri::AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .blocking_pick_folder()
        .map(|p| p.to_string())
}

// ── Scanner commands ──────────────────────────────────────────────────────────

#[tauri::command]
async fn scan_music_folder(path: String, app: tauri::AppHandle) -> Result<String, String> {
    let app_clone = app.clone();
    let path_clone = path.clone();

    tokio::task::spawn_blocking(move || {
        scan_folder_streaming(&path_clone, &app_clone);
    })
    .await
    .map_err(|e| e.to_string())?;

    let size = tokio::task::spawn_blocking(move || calculate_library_size(&path))
        .await
        .map_err(|e| e.to_string())?;

    Ok(format_size(size))
}

#[tauri::command]
async fn get_library_size(path: String) -> String {
    let bytes = tokio::task::spawn_blocking(move || calculate_library_size(&path))
        .await
        .unwrap_or(0);
    format_size(bytes)
}

fn format_size(bytes: u64) -> String {
    const GB: u64 = 1_073_741_824;
    const MB: u64 = 1_048_576;
    if bytes >= GB {
        format!("{:.3} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.3} MB", bytes as f64 / MB as f64)
    }
}

// ── Library cache commands ────────────────────────────────────────────────────

#[tauri::command]
fn save_library_cache(data: String, app: tauri::AppHandle) -> Result<(), String> {
    let base_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;

    // Parse incoming albums and split into meta (no cover_art) + covers map
    let albums: Vec<serde_json::Value> = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let mut covers: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    let meta: Vec<serde_json::Value> = albums.into_iter().map(|mut album| {
        if let Some(obj) = album.as_object_mut() {
            if let Some(id) = obj.get("id").and_then(|v| v.as_str()).map(String::from) {
                if let Some(cover) = obj.remove("cover_art") {
                    if !cover.is_null() {
                        covers.insert(id, cover);
                    }
                }
                obj.insert("cover_art".to_string(), serde_json::Value::Null);
            }
        }
        album
    }).collect();

    std::fs::write(base_dir.join("library_meta.json"), serde_json::to_string(&meta).unwrap())
        .map_err(|e| e.to_string())?;
    std::fs::write(base_dir.join("library_covers.json"), serde_json::to_string(&covers).unwrap())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn load_library_cache(app: tauri::AppHandle) -> Result<bool, String> {
    let base_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let meta_path = base_dir.join("library_meta.json");

    let meta_data = match std::fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(_) => return Ok(false),
    };

    let albums: Vec<serde_json::Value> = match serde_json::from_str(&meta_data) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    if albums.is_empty() {
        return Ok(false);
    }

    // Phase 1: emit all albums without covers — instant
    for album in &albums {
        app.emit("scan:album", album).ok();
    }
    app.emit("scan:done", ()).ok();

    // Phase 2: stream covers in background
    let covers_path = base_dir.join("library_covers.json");
    let app2 = app.clone();
    tokio::task::spawn_blocking(move || {
        let Ok(covers_data) = std::fs::read_to_string(&covers_path) else { return };
        let Ok(covers) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&covers_data) else { return };
        for (id, cover_art) in covers {
            app2.emit("cache:cover", serde_json::json!({ "id": id, "cover_art": cover_art })).ok();
        }
    });

    Ok(true)
}

// ── Audio commands ────────────────────────────────────────────────────────────

#[tauri::command]
fn audio_play(
    path: String,
    duration: f64,
    player: tauri::State<SharedPlayer>,
) -> Result<(), String> {
    player.play(&path, duration)
}

#[tauri::command]
fn audio_pause(player: tauri::State<SharedPlayer>) {
    player.pause();
}

#[tauri::command]
fn audio_resume(player: tauri::State<SharedPlayer>) {
    player.resume();
}

#[tauri::command]
fn audio_stop(player: tauri::State<SharedPlayer>) {
    player.stop();
}

#[tauri::command]
fn audio_set_volume(volume: f32, player: tauri::State<SharedPlayer>) {
    player.set_volume(volume);
}

#[tauri::command]
fn audio_get_state(player: tauri::State<SharedPlayer>) -> PlaybackState {
    player.get_state()
}

#[tauri::command]
fn audio_is_finished(player: tauri::State<SharedPlayer>) -> bool {
    player.is_finished()
}

#[tauri::command]
fn audio_get_position(player: tauri::State<SharedPlayer>) -> f64 {
    player.get_position()
}

// ── App entry ─────────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let player = create_player();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(player)
        .invoke_handler(tauri::generate_handler![
            pick_folder,
            scan_music_folder,
            get_library_size,
            audio_play,
            audio_pause,
            audio_resume,
            audio_stop,
            audio_set_volume,
            audio_get_state,
            audio_is_finished,
            audio_get_position,
            save_library_cache,
            load_library_cache,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
