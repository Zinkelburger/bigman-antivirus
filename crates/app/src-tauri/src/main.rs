//! Bigman desktop app: a Tauri shell whose only job is to forward typed requests
//! to bigmand and show the results. No checks run in this process.
mod daemon;
mod files;
mod instance;
mod tray;

use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_window_state::StateFlags;

fn main() {
    // `--socket PATH` points the app (and any bigmand it starts) at a separate daemon,
    // as tests and extra profiles need. It is passed on as BIGMAN_SOCKET.
    let args: Vec<String> = std::env::args().collect();
    if let Some(path) = args
        .windows(2)
        .find(|w| w[0] == "--socket")
        .map(|w| w[1].clone())
    {
        std::env::set_var("BIGMAN_SOCKET", path);
    }
    // `--background` starts in the tray with the window hidden, for starting at login.
    let background = args.iter().any(|a| a == "--background");
    // Bigman is already in the tray: it shows its window and this launch is done.
    let later_launches = match instance::claim() {
        instance::Claim::AlreadyRunning => return,
        instance::Claim::First(listener) => listener,
    };
    // WebKitGTK's documented fallback for blank or flickering windows on NVIDIA drivers.
    if std::path::Path::new("/proc/driver/nvidia/version").exists()
        && std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none()
    {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    tauri::Builder::default()
        // Size and position are remembered. Visibility is not: the window starts
        // hidden and is shown below, so a launch after the app was last closed to
        // the tray still opens a window.
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::all() & !StateFlags::VISIBLE)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(daemon::Launcher::default()))
        .invoke_handler(tauri::generate_handler![
            daemon::daemon_request,
            daemon::daemon_cancel,
            daemon::daemon_subscribe,
            daemon::stop_and_quit,
            files::choose_path,
            files::export_report
        ])
        .on_window_event(|window, event| {
            // With a tray to come back from, closing the window leaves Bigman there,
            // as the tray's two Quit entries say. Without one, closing quits.
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if tray::desktop_has_tray() {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .setup(move |app| {
            tray::install(app.handle())?;
            if let Some(listener) = later_launches {
                let handle = app.handle().clone();
                instance::serve(listener, move || show_window(&handle));
            }
            if !background {
                show_window(app.handle());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Bigman could not start its window");
}

/// Brings the one window to the front, from the tray, a second launch or startup.
pub(crate) fn show_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}
