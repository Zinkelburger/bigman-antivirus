//! Browser native messaging uses bounded stdio frames. Results are persisted once;
//! the desktop reads the same history, so no listening socket or duplicate scan is needed.
use crate::{model::Report, runner::Cancel, store::Store};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
};
const MAX_MESSAGE: usize = 64 * 1024;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    #[serde(default)]
    pub op: String,
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    pub url: Option<String>,
}
#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub reason: String,
    pub report: Option<Report>,
}
pub fn read_frame(input: &mut impl Read) -> Result<Request> {
    let mut length = [0; 4];
    input.read_exact(&mut length)?;
    let n = u32::from_ne_bytes(length) as usize;
    if n == 0 || n > MAX_MESSAGE {
        bail!("Invalid native message size");
    }
    let mut bytes = vec![0; n];
    input.read_exact(&mut bytes)?;
    Ok(serde_json::from_slice(&bytes)?)
}
pub fn handle(store: &Store, req: Request, stop: &Cancel) -> Result<Response> {
    if req.op == "health" {
        return Ok(Response {
            status: "AVAILABLE".into(),
            reason:
                "Native host reachable. Scanner availability and coverage are reported per job."
                    .into(),
            report: None,
        });
    }
    let report = if req.op == "inspect-url" {
        let url = url::Url::parse(
            req.url
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("Missing URL"))?,
        )?;
        if url.scheme() != "https" {
            bail!("Native installer previews require HTTPS URLs; select local scripts in the desktop app");
        }
        crate::inspect::inspect(
            store,
            req.url
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("Missing URL"))?,
            stop,
        )?
    } else {
        let p = PathBuf::from(
            req.file_path
                .ok_or_else(|| anyhow::anyhow!("Missing filePath"))?,
        );
        if std::fs::symlink_metadata(&p)?.file_type().is_symlink() {
            bail!("Download path must not be a symlink");
        }
        let p = std::fs::canonicalize(p)?;
        let downloads = std::fs::canonicalize(crate::setup::downloads())?;
        if !p.starts_with(downloads) || !p.is_file() {
            bail!("Browser scans are restricted to regular files in ~/Downloads. Open other files explicitly in the desktop app.");
        }
        match req.op.as_str() {
            "" | "scan" => crate::scanner::scan(store, &p, stop)?,
            "pdf" => crate::pdf_scanner::scan_pdf(store, &p, stop)?,
            "inspect-file" => crate::inspect::inspect(store, &p.to_string_lossy(), stop)?,
            _ => bail!("Unsupported native operation"),
        }
    };
    Ok(Response {
        status: if !report.errors.is_empty() {
            "INCOMPLETE"
        } else if !report.findings.is_empty() {
            "FINDINGS"
        } else {
            "NO_FINDINGS"
        }
        .into(),
        reason: report.status().into(),
        report: Some(report),
    })
}
pub fn host(store: &Store, stop: &Cancel) -> Result<()> {
    let response = match read_frame(&mut std::io::stdin()).and_then(|r| handle(store, r, stop)) {
        Ok(r) => r,
        Err(e) => Response {
            status: "INCOMPLETE".into(),
            reason: e.to_string(),
            report: None,
        },
    };
    let mut bytes = serde_json::to_vec(&response)?;
    if bytes.len() > 900 * 1024 {
        bytes = serde_json::to_vec(&Response {
            status: response.status,
            reason: format!(
                "{}. Full report is available in desktop history.",
                response.reason
            ),
            report: None,
        })?;
    }
    let mut out = std::io::stdout().lock();
    out.write_all(&(bytes.len() as u32).to_ne_bytes())?;
    out.write_all(&bytes)?;
    out.flush()?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oversized_native_frames_rejected() {
        let mut b = (MAX_MESSAGE as u32 + 1).to_ne_bytes().as_slice().to_owned();
        assert!(read_frame(&mut b.as_slice()).is_err());
        b = 10u32.to_ne_bytes().to_vec();
        assert!(read_frame(&mut b.as_slice()).is_err());
    }
    #[test]
    fn unknown_fields_rejected() {
        let b = br#"{"op":"health","command":"arbitrary"}"#;
        let mut frame = (b.len() as u32).to_ne_bytes().to_vec();
        frame.extend(b);
        assert!(read_frame(&mut frame.as_slice()).is_err());
    }
}
