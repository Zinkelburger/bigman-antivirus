fn main() {
    // The webview may call only the commands listed here; capabilities/default.json
    // grants them to the main window.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "daemon_request",
            "daemon_cancel",
            "choose_path",
            "export_report",
        ]),
    ))
    .expect("Tauri build configuration is invalid");
}
