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
    let covers_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("covers");
    std::fs::create_dir_all(&covers_dir).map_err(|e| e.to_string())?;

    let app_clone = app.clone();
    let path_clone = path.clone();

    tokio::task::spawn_blocking(move || {
        scan_folder_streaming(&path_clone, &app_clone, &covers_dir);
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
    let path = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("library_cache.json");
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(&path, data).map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_library_cache(app: tauri::AppHandle) -> Result<bool, String> {
    let path = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("library_cache.json");

    let data = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Ok(false),
    };

    let albums: Vec<serde_json::Value> = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    if albums.is_empty() {
        return Ok(false);
    }

    for album in albums {
        app.emit("scan:album", &album).ok();
    }
    app.emit("scan:done", ()).ok();

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
