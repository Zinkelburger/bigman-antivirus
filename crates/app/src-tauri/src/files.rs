//! The two places the app touches files itself: the desktop's file chooser, and
//! saving a report the user chose to export. Paths are validated in core when used.
use crate::daemon::{Launcher, RequestError};
use bigman_core::ipc::{Request, Response};
use std::{io::Write, os::unix::fs::OpenOptionsExt, path::PathBuf, sync::Arc};

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathKind {
    File,
    Directory,
}

/// Opens the desktop file chooser (through the XDG portal where available).
#[tauri::command]
pub async fn choose_path(kind: PathKind, title: String) -> Option<PathBuf> {
    let dialog = rfd::AsyncFileDialog::new().set_title(&title);
    let chosen = match kind {
        PathKind::File => dialog.pick_file().await,
        PathKind::Directory => dialog.pick_folder().await,
    };
    chosen.map(|handle| handle.path().to_path_buf())
}

/// Saves a report's full JSON where the user chooses. Nothing leaves the machine.
#[tauri::command]
pub async fn export_report(
    launcher: tauri::State<'_, Arc<Launcher>>,
    id: String,
) -> Result<Option<PathBuf>, RequestError> {
    let launcher = launcher.inner().clone();
    let request_id = id.clone();
    let response = tauri::async_runtime::spawn_blocking(move || {
        launcher.request(Request::Report { id: request_id })
    })
    .await
    .map_err(|e| RequestError::Failed(e.to_string()))??;
    let report = match response {
        Response::Report(report) => report,
        Response::Error(e) => return Err(RequestError::Failed(e.message)),
        _ => return Err(RequestError::Failed("Unexpected daemon response".into())),
    };
    let Some(target) = rfd::AsyncFileDialog::new()
        .set_title("Export report")
        .set_file_name(format!("bigman-{id}.json"))
        .save_file()
        .await
    else {
        return Ok(None);
    };
    let path = target.path().to_path_buf();
    let json =
        serde_json::to_vec_pretty(&report).map_err(|e| RequestError::Failed(e.to_string()))?;
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)
        .and_then(|mut f| f.write_all(&json))
        .map_err(|e| RequestError::Failed(format!("Cannot write {}: {e}", path.display())))?;
    Ok(Some(path))
}
