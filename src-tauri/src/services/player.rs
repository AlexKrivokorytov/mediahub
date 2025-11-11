use std::io::Cursor;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use rodio::{OutputStream, Sink, Decoder};

pub enum PlayerCmd {
    PlayUrl(String, f32),
    Pause,
    Resume,
    SetVolume(f32),
    Stop,
}

#[derive(Clone)]
pub struct PlayerHandle {
    tx: Sender<PlayerCmd>,
}

impl PlayerHandle {
    pub fn play_url(&self, url: String, volume: f32) -> Result<(), String> {
        self.tx.send(PlayerCmd::PlayUrl(url, volume)).map_err(|e| e.to_string())
    }
    pub fn pause(&self) -> Result<(), String> { self.tx.send(PlayerCmd::Pause).map_err(|e| e.to_string()) }
    pub fn resume(&self) -> Result<(), String> { self.tx.send(PlayerCmd::Resume).map_err(|e| e.to_string()) }
    pub fn set_volume(&self, v: f32) -> Result<(), String> { self.tx.send(PlayerCmd::SetVolume(v)).map_err(|e| e.to_string()) }
}

pub fn start_audio_engine() -> PlayerHandle {
    let (tx, rx): (Sender<PlayerCmd>, Receiver<PlayerCmd>) = channel();
    thread::spawn(move || {
        // Non-Send rodio objects live on this dedicated thread
        let (_stream, handle) = match OutputStream::try_default() {
            Ok(pair) => pair,
            Err(e) => { eprintln!("[audio] Output stream error: {}", e); return; }
        };
        let mut sink: Option<Sink> = None;
        let mut volume: f32 = 0.5;
        while let Ok(cmd) = rx.recv() {
            match cmd {
                PlayerCmd::PlayUrl(url, vol) => {
                    volume = vol.clamp(0.0, 1.0);
                    // Download on this thread (blocking) — tracks should be short (previews)
                    match reqwest::blocking::get(&url).and_then(|r| r.error_for_status()).and_then(|r| r.bytes().map_err(|e| e.into())) {
                        Ok(bytes) => {
                            let cursor = Cursor::new(bytes);
                            match Decoder::new(cursor) {
                                Ok(source) => {
                                    let s = Sink::try_new(&handle).unwrap_or_else(|_| Sink::new_idle().0);
                                    s.set_volume(volume);
                                    s.append(source);
                                    s.play();
                                    sink = Some(s);
                                }
                                Err(e) => eprintln!("[audio] Decoder error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("[audio] Download error: {}", e),
                    }
                }
                PlayerCmd::Pause => { if let Some(ref s) = sink { s.pause(); } }
                PlayerCmd::Resume => { if let Some(ref s) = sink { s.play(); } }
                PlayerCmd::SetVolume(v) => { volume = v.clamp(0.0, 1.0); if let Some(ref s) = sink { s.set_volume(volume); } }
                PlayerCmd::Stop => { if let Some(s) = sink.take() { s.stop(); } }
            }
        }
    });
    PlayerHandle { tx }
}


