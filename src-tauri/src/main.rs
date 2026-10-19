#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io,
    process::{Child, Command, Stdio},
    sync::Mutex,
};

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::{Manager, State, WindowEvent};

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::HWND,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CreateWindowExW, SetWindowPos, ShowWindow, HWND_TOP, SW_HIDE, SW_SHOW, SWP_NOACTIVATE,
        SWP_NOOWNERZORDER, WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS,
    },
};

struct MpvState {
    child: Mutex<Option<Child>>,
    // HWND is stored as an integer so application state remains Send + Sync.
    video_host: Mutex<Option<isize>>,
}

#[cfg(windows)]
fn create_video_host(parent: isize) -> Result<isize, String> {
    let class_name: Vec<u16> = "STATIC\0".encode_utf16().collect();
    let window_name: Vec<u16> = "EliteStocks TV embedded video surface\0"
        .encode_utf16()
        .collect();

    let host = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_CHILD | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
            0,
            0,
            1,
            1,
            parent as HWND,
            std::ptr::null_mut(),
            GetModuleHandleW(std::ptr::null()),
            std::ptr::null(),
        )
    };

    if host.is_null() {
        return Err(format!(
            "Could not create the native video surface: {}",
            io::Error::last_os_error()
        ));
    }

    Ok(host as isize)
}

#[cfg(windows)]
fn hide_video_host(state: &MpvState) {
    if let Ok(host) = state.video_host.lock() {
        if let Some(hwnd) = *host {
            unsafe {
                ShowWindow(hwnd as HWND, SW_HIDE);
            }
        }
    }
}

fn validate_stream_url(stream_url: &str) -> Result<&str, String> {
    let value = stream_url.trim();
    if value.is_empty() {
        return Err("Enter a stream URL first.".into());
    }
    if value
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err("The stream URL cannot contain spaces or control characters.".into());
    }

    let lowered = value.to_ascii_lowercase();
    let allowed_schemes = ["http://", "https://", "rtsp://", "rtmp://", "udp://"];
    if !allowed_schemes.iter().any(|scheme| lowered.starts_with(scheme)) {
        return Err("Unsupported URL scheme. Use HTTP, HTTPS, RTSP, RTMP, or UDP.".into());
    }
    Ok(value)
}

#[cfg(windows)]
#[tauri::command]
fn set_video_bounds(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    scale_factor: f64,
    visible: bool,
    state: State<'_, MpvState>,
) -> Result<(), String> {
    if ![x, y, width, height, scale_factor].iter().all(|value| value.is_finite())
        || scale_factor <= 0.0
    {
        return Err("Invalid video surface dimensions.".into());
    }

    let host = *state
        .video_host
        .lock()
        .map_err(|_| "Video surface state is unavailable.".to_string())?
        .as_ref()
        .ok_or_else(|| "The native video surface has not been initialized.".to_string())?;

    let px = (x * scale_factor).round().clamp(-10000.0, 10000.0) as i32;
    let py = (y * scale_factor).round().clamp(-10000.0, 10000.0) as i32;
    let pw = (width * scale_factor).round().clamp(1.0, 10000.0) as i32;
    let ph = (height * scale_factor).round().clamp(1.0, 10000.0) as i32;

    let result = unsafe {
        SetWindowPos(
            host as HWND,
            HWND_TOP,
            px,
            py,
            pw,
            ph,
            SWP_NOACTIVATE | SWP_NOOWNERZORDER,
        )
    };
    if result == 0 {
        return Err(format!(
            "Could not resize the embedded video surface: {}",
            io::Error::last_os_error()
        ));
    }

    unsafe {
        ShowWindow(host as HWND, if visible { SW_SHOW } else { SW_HIDE });
    }

    Ok(())
}

#[cfg(not(windows))]
#[tauri::command]
fn set_video_bounds(
    _x: f64,
    _y: f64,
    _width: f64,
    _height: f64,
    _scale_factor: f64,
    _visible: bool,
) -> Result<(), String> {
    Err("Embedded MPV video output is currently supported on Windows only.".into())
}

#[cfg(windows)]
#[tauri::command]
fn play_stream(stream_url: String, state: State<'_, MpvState>) -> Result<String, String> {
    let url = validate_stream_url(&stream_url)?;

    let host = *state
        .video_host
        .lock()
        .map_err(|_| "Video surface state is unavailable.".to_string())?
        .as_ref()
        .ok_or_else(|| "The native video surface has not been initialized.".to_string())?;

    let mut active = state
        .child
        .lock()
        .map_err(|_| "Playback state is unavailable.".to_string())?;

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

    unsafe {
        ShowWindow(host as HWND, SW_SHOW);
    }

    // MPV creates its video output as a child of the native host through --wid.
    let child = match Command::new("mpv.exe")
        .arg(format!("--wid={}", host as u32))
        .arg("--force-window=yes")
        .arg("--title=EliteStocks TV")
        .arg("--no-terminal")
        .arg("--")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            unsafe {
                ShowWindow(host as HWND, SW_HIDE);
            }
            return Err(format!(
                "Could not start MPV. Install MPV and ensure mpv.exe is available on your Windows PATH. Details: {error}"
            ));
        }
    };

    *active = Some(child);
    Ok("Playback started in the embedded video surface.".into())
}

#[cfg(not(windows))]
#[tauri::command]
fn play_stream(_stream_url: String) -> Result<String, String> {
    Err("Embedded MPV playback is currently supported on Windows only.".into())
}

#[tauri::command]
fn stop_stream(state: State<'_, MpvState>) -> Result<String, String> {
    let mut active = state
        .child
        .lock()
        .map_err(|_| "Playback state is unavailable.".to_string())?;

    let stop_result = if let Some(mut child) = active.take() {
        match child.try_wait() {
            Ok(Some(_)) => Ok("Playback has already stopped.".to_string()),
            Ok(None) => {
                child
                    .kill()
                    .map_err(|error| format!("Could not stop MPV: {error}"))?;
                let _ = child.wait();
                Ok("MPV playback stopped.".to_string())
            }
            Err(error) => Err(format!("Could not check MPV status: {error}")),
        }
    } else {
        Ok("No MPV playback process is running.".to_string())
    };

    #[cfg(windows)]
    hide_video_host(&state);

    stop_result
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(windows)]
            let host = {
                let main_window = app.get_webview_window("main").ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "Main application window is missing")
                })?;
                let native_handle = main_window.window_handle().map_err(|error| {
                    io::Error::new(io::ErrorKind::Other, error.to_string())
                })?;

                let parent = match native_handle.as_raw() {
                    RawWindowHandle::Win32(handle) => handle.hwnd.get(),
                    _ => {
                        return Err(Box::new(io::Error::new(
                            io::ErrorKind::Other,
                            "The embedded MPV surface requires a Win32 application window",
                        )));
                    }
                };
                create_video_host(parent).map_err(|message| {
                    io::Error::new(io::ErrorKind::Other, message)
                })?
            };

            app.manage(MpvState {
                child: Mutex::new(None),
                video_host: Mutex::new({
                    #[cfg(windows)]
                    {
                        Some(host)
                    }
                    #[cfg(not(windows))]
                    {
                        None
                    }
                }),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_video_bounds, play_stream, stop_stream])
        .on_window_event(|window, event| {
            if matches!(event, WindowEvent::Destroyed) {
                let state = window.state::<MpvState>();
                if let Ok(mut active) = state.child.lock() {
                    if let Some(mut child) = active.take() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run EliteStocks TV Desktop");
}
