#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    process::{Child, Command, Stdio},
    sync::Mutex,
};
use tauri::State;

struct MpvProcess(Mutex<Option<Child>>);

fn validate_stream_url(stream_url: &str) -> Result<&str, String> {
    let value = stream_url.trim();
    if value.is_empty() {
        return Err("Enter a stream URL first.".into());
    }
    if value.chars().any(|character| character.is_control() || character.is_whitespace()) {
        return Err("The stream URL cannot contain spaces or control characters.".into());
    }

    let lowered = value.to_ascii_lowercase();
    let allowed_schemes = ["http://", "https://", "rtsp://", "rtmp://", "udp://"];
    if !allowed_schemes.iter().any(|scheme| lowered.starts_with(scheme)) {
        return Err("Unsupported URL scheme. Use HTTP, HTTPS, RTSP, RTMP, or UDP.".into());
    }
    Ok(value)
}

#[tauri::command]
fn play_stream(stream_url: String, player: State<'_, MpvProcess>) -> Result<String, String> {
    let url = validate_stream_url(&stream_url)?;
    let mut active = player.0.lock().map_err(|_| "Playback state is unavailable.".to_string())?;

    if let Some(child) = active.as_mut() {
        match child.try_wait() {
            Ok(Some(_)) => {}
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
            }
            Err(error) => return Err(format!("Could not check the current MPV process: {error}")),
        }
    }
    *active = None;

    let child = Command::new("mpv.exe")
        .arg("--force-window=yes")
        .arg("--title=EliteStocks TV")
        .arg("--")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| {
            format!(
                "Could not start MPV. Install MPV and ensure mpv.exe is available on your Windows PATH. Details: {error}"
            )
        })?;

    *active = Some(child);
    Ok("MPV opened the stream in its own window. Embedded playback is planned for a later milestone.".into())
}

#[tauri::command]
fn stop_stream(player: State<'_, MpvProcess>) -> Result<String, String> {
    let mut active = player.0.lock().map_err(|_| "Playback state is unavailable.".to_string())?;
    if let Some(mut child) = active.take() {
        match child.try_wait() {
            Ok(Some(_)) => Ok("Playback has already stopped.".into()),
            Ok(None) => {
                child.kill().map_err(|error| format!("Could not stop MPV: {error}"))?;
                let _ = child.wait();
                Ok("MPV playback stopped.".into())
            }
            Err(error) => Err(format!("Could not check MPV status: {error}")),
        }
    } else {
        Ok("No MPV playback process is running.".into())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(MpvProcess(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![play_stream, stop_stream])
        .run(tauri::generate_context!())
        .expect("failed to run EliteStocks TV Desktop");
}
