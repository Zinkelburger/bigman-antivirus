//! Bigman desktop app: a Tauri shell whose only job is to forward typed requests
//! to bigmand and show the results. No checks run in this process.
mod daemon;
mod files;
mod tray;

use std::sync::Arc;

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
    // WebKitGTK's documented fallback for blank or flickering windows on NVIDIA drivers.
    if std::path::Path::new("/proc/driver/nvidia/version").exists()
        && std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none()
    {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(daemon::Launcher::default()))
        .invoke_handler(tauri::generate_handler![
            daemon::daemon_request,
            daemon::daemon_cancel,
            daemon::daemon_subscribe,
            files::choose_path,
            files::export_report
        ])
        .setup(|app| {
            tray::install(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Bigman could not start its window");
}
