use std::io::Cursor;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink};
use crate::state::AppState;

const SOUND_TOUDI: &[u8] = include_bytes!("../sounds/投递.mp3");
const SOUND_BO: &[u8]     = include_bytes!("../sounds/啵.mp3");
const SOUND_WA: &[u8]     = include_bytes!("../sounds/WhatsApp.mp3");
const SOUND_LANG: &[u8]   = include_bytes!("../sounds/太空狼人杀.mp3");
const SOUND_PIKE: &[u8]   = include_bytes!("../sounds/皮克通恩.mp3");

fn get_sound_bytes(name: &str) -> Option<&[u8]> {
    match name {
        "投递" => Some(SOUND_TOUDI),
        "啵" => Some(SOUND_BO),
        "WhatsApp" => Some(SOUND_WA),
        "太空狼人杀" => Some(SOUND_LANG),
        "皮克通恩" => Some(SOUND_PIKE),
        _ => Some(SOUND_TOUDI),
    }
}

pub fn play_received_sound(state: &Arc<AppState>) {
    let enabled = *state.sound_enabled.lock().unwrap();
    if !enabled { return; }
    let name = state.sound_name.lock().unwrap().clone();
    let data = match get_sound_bytes(&name) {
        Some(d) => d.to_vec(),
        None => return,
    };
    std::thread::spawn(move || {
        let cursor = Cursor::new(data);
        if let Ok(src) = Decoder::new(cursor) {
            if let Ok((_stream, handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&handle) {
                    sink.append(src);
                    sink.sleep_until_end();
                }
            }
        }
    });
}
