use rodio::{Decoder, OutputStream, Sink, Source};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::panic;

// Commands sent to the audio thread
enum Cmd {
    Play { path: String, duration: f64 },
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

#[derive(Debug, Clone)]
struct Inner {
    is_playing: bool,
    is_paused: bool,
    current_path: Option<String>,
    duration_secs: f64,
    volume: f32,
    // position tracking
    play_started_at: Option<Instant>,
    elapsed_before_pause: f64,
}

impl Inner {
    fn new() -> Self {
        Inner {
            is_playing: false,
            is_paused: false,
            current_path: None,
            duration_secs: 0.0,
            volume: 1.0,
            play_started_at: None,
            elapsed_before_pause: 0.0,
        }
    }

    fn position(&self) -> f64 {
        let running = self.play_started_at
            .map(|t| t.elapsed().as_secs_f64())
            .unwrap_or(0.0);
        (self.elapsed_before_pause + running).min(self.duration_secs)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub is_paused: bool,
    pub current_path: Option<String>,
    pub duration_secs: f64,
    pub volume: f32,
    pub position_secs: f64,
}

// ── Streaming M4A source ───────────────────────────────────────────────────────
//
// Wraps symphonia's format reader + decoder as a lazy rodio::Source.
// Decodes one AAC packet at a time as rodio pulls samples, so playback
// starts immediately without loading the entire file into memory.

struct M4aSource {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    track_id: u32,
    buffer: Vec<f32>,
    buf_pos: usize,
    channels: u16,
    sample_rate: u32,
    done: bool,
}

impl M4aSource {
    fn open(path: &str) -> Option<Self> {
        use symphonia::core::formats::FormatOptions;
        use symphonia::core::io::MediaSourceStream;
        use symphonia::core::meta::MetadataOptions;
        use symphonia::core::probe::Hint;

        let file = std::fs::File::open(path).ok()?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        hint.with_extension("m4a");

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
            .map_err(|e| eprintln!("[audio] m4a probe failed: {e}"))
            .ok()?;

        let format = probed.format;
        let track = format.default_track()
            .map(|t| t.clone())
            .or_else(|| { eprintln!("[audio] m4a: no default track"); None })?;

        let track_id = track.id;
        // Read spec from codec params — available after probing, before decoding
        let channels = track.codec_params.channels
            .map(|c| c.count() as u16)
            .unwrap_or(2);
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &symphonia::core::codecs::DecoderOptions::default())
            .map_err(|e| eprintln!("[audio] m4a: codec init failed: {e}"))
            .ok()?;

        eprintln!("[audio] m4a opened: {}Hz {}ch", sample_rate, channels);
        Some(M4aSource {
            format,
            decoder,
            track_id,
            buffer: Vec::new(),
            buf_pos: 0,
            channels,
            sample_rate,
            done: false,
        })
    }

    // Decode the next packet and fill the sample buffer.
    fn fill_buffer(&mut self) {
        use symphonia::core::audio::SampleBuffer;

        self.buffer.clear();
        self.buf_pos = 0;

        while self.buffer.is_empty() && !self.done {
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => { self.done = true; break; }
            };
            if packet.track_id() != self.track_id { continue; }

            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();
                    // Update channels/rate from actual decoded data (more reliable than codec_params)
                    self.channels = spec.channels.count() as u16;
                    self.sample_rate = spec.rate;
                    let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                    buf.copy_interleaved_ref(decoded);
                    self.buffer.extend_from_slice(buf.samples());
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(_) => { self.done = true; break; }
            }
        }
    }
}

impl Iterator for M4aSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.buf_pos >= self.buffer.len() {
            if self.done { return None; }
            self.fill_buffer();
            if self.buffer.is_empty() { return None; }
        }
        let sample = self.buffer[self.buf_pos];
        self.buf_pos += 1;
        Some(sample)
    }
}

impl Source for M4aSource {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { self.channels }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}

// ── AudioPlayer ────────────────────────────────────────────────────────────────

pub struct AudioPlayer {
    tx: Sender<Cmd>,
    state: Arc<Mutex<Inner>>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Cmd>();
        let state = Arc::new(Mutex::new(Inner::new()));
        let state_thread = Arc::clone(&state);

        std::thread::spawn(move || {
            let Ok((_stream, handle)) = OutputStream::try_default() else {
                return;
            };
            let mut sink: Option<Sink> = None;

            loop {
                // Poll every 500ms to detect natural track end
                let cmd = match rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(cmd) => Some(cmd),
                    Err(RecvTimeoutError::Timeout) => {
                        if let Some(s) = &sink {
                            if s.empty() {
                                let mut st = state_thread.lock().unwrap();
                                if st.is_playing {
                                    st.is_playing = false;
                                    st.is_paused = false;
                                }
                            }
                        }
                        None
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                };

                let Some(cmd) = cmd else { continue };
                match cmd {
                    Cmd::Play { path, duration } => {
                        if let Some(s) = sink.take() { s.stop(); }
                        eprintln!("[audio] Play: {path}");

                        let is_m4a = path.to_lowercase().ends_with(".m4a");

                        if is_m4a {
                            // Open file and init symphonia reader — fast (no decoding yet).
                            // Decoding happens lazily as rodio pulls samples.
                            match M4aSource::open(&path) {
                                Some(source) => {
                                    match Sink::try_new(&handle) {
                                        Ok(s) => {
                                            let vol = state_thread.lock().unwrap().volume;
                                            s.set_volume(vol);
                                            s.append(source);
                                            let mut st = state_thread.lock().unwrap();
                                            st.is_playing = true;
                                            st.is_paused = false;
                                            st.current_path = Some(path);
                                            st.duration_secs = duration;
                                            st.play_started_at = Some(Instant::now());
                                            st.elapsed_before_pause = 0.0;
                                            sink = Some(s);
                                        }
                                        Err(_) => {}
                                    }
                                }
                                None => { eprintln!("[audio] m4a open failed: {path}"); }
                            }
                        } else {
                            match std::fs::read(&path) {
                                Ok(data) => {
                                    let cursor = std::io::Cursor::new(data);
                                    let decoder_result = panic::catch_unwind(|| Decoder::new(cursor));
                                    let source = match decoder_result {
                                        Ok(Ok(s)) => Some(s),
                                        Ok(Err(e)) => { eprintln!("[audio] Decoder error on {path}: {e}"); None }
                                        Err(_)     => { eprintln!("[audio] Decoder PANIC on: {path}"); None }
                                    };
                                    if let Some(source) = source {
                                        match Sink::try_new(&handle) {
                                            Ok(s) => {
                                                let vol = state_thread.lock().unwrap().volume;
                                                s.set_volume(vol);
                                                s.append(source);
                                                let mut st = state_thread.lock().unwrap();
                                                st.is_playing = true;
                                                st.is_paused = false;
                                                st.current_path = Some(path);
                                                st.duration_secs = duration;
                                                st.play_started_at = Some(Instant::now());
                                                st.elapsed_before_pause = 0.0;
                                                sink = Some(s);
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Cmd::Pause => {
                        if let Some(s) = &sink {
                            s.pause();
                            let mut st = state_thread.lock().unwrap();
                            let running = st.play_started_at
                                .map(|t| t.elapsed().as_secs_f64())
                                .unwrap_or(0.0);
                            st.elapsed_before_pause += running;
                            st.play_started_at = None;
                            st.is_playing = false;
                            st.is_paused = true;
                        }
                    }
                    Cmd::Resume => {
                        if let Some(s) = &sink {
                            s.play();
                            let mut st = state_thread.lock().unwrap();
                            st.play_started_at = Some(Instant::now());
                            st.is_playing = true;
                            st.is_paused = false;
                        }
                    }
                    Cmd::Stop => {
                        if let Some(s) = sink.take() { s.stop(); }
                        let mut st = state_thread.lock().unwrap();
                        st.is_playing = false;
                        st.is_paused = false;
                        st.current_path = None;
                        st.duration_secs = 0.0;
                        st.play_started_at = None;
                        st.elapsed_before_pause = 0.0;
                    }
                    Cmd::SetVolume(v) => {
                        let vol = v.clamp(0.0, 1.0);
                        if let Some(s) = &sink { s.set_volume(vol); }
                        state_thread.lock().unwrap().volume = vol;
                    }
                }
            }
        });

        AudioPlayer { tx, state }
    }

    pub fn play(&self, path: &str, duration: f64) -> Result<(), String> {
        self.tx.send(Cmd::Play { path: path.to_string(), duration })
            .map_err(|e| e.to_string())
    }

    pub fn pause(&self) { let _ = self.tx.send(Cmd::Pause); }
    pub fn resume(&self) { let _ = self.tx.send(Cmd::Resume); }
    pub fn stop(&self) { let _ = self.tx.send(Cmd::Stop); }
    pub fn set_volume(&self, volume: f32) { let _ = self.tx.send(Cmd::SetVolume(volume)); }

    pub fn is_finished(&self) -> bool {
        let st = self.state.lock().unwrap();
        !st.is_playing && !st.is_paused && st.current_path.is_some()
    }

    pub fn get_position(&self) -> f64 {
        self.state.lock().unwrap().position()
    }

    pub fn get_state(&self) -> PlaybackState {
        let st = self.state.lock().unwrap();
        PlaybackState {
            is_playing: st.is_playing,
            is_paused: st.is_paused,
            current_path: st.current_path.clone(),
            duration_secs: st.duration_secs,
            volume: st.volume,
            position_secs: st.position(),
        }
    }
}

pub type SharedPlayer = Arc<AudioPlayer>;

pub fn create_player() -> SharedPlayer {
    Arc::new(AudioPlayer::new())
}
