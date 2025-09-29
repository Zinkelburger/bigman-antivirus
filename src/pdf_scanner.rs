// src/pdf_scanner.rs

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::fs;
use std::path::Path;

/// The result of a single PDF scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfScanResult {
    pub file_path: String,
    pub timestamp: u64,
    pub is_suspicious: bool,
    pub reason: String,
    pub scan_id: String,
    pub pdfid_output: Option<String>,
    pub metadata: Option<String>,
}

/// Runs pdfid.py on the file if available
fn run_pdfid(file_path: &str) -> Option<String> {
    Command::new("pdfid.py")
        .arg(file_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                None
            }
        })
}

/// Extracts PDF metadata using pdfinfo if available
fn get_pdf_metadata(file_path: &str) -> Option<String> {
    Command::new("pdfinfo")
        .arg(file_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                None
            }
        })
}

/// Saves scan result to a file in pdf_scans directory
fn save_scan_result(result: &PdfScanResult) {
    let scan_dir = Path::new("pdf_scans");
    if !scan_dir.exists() {
        let _ = fs::create_dir_all(scan_dir);
    }

    let scan_file = scan_dir.join(format!("{}.json", result.scan_id));
    let _ = fs::write(scan_file, serde_json::to_string_pretty(result).unwrap_or_default());
}

/// Scans a PDF file for suspicious auto-action tags.
pub fn scan_pdf_for_actions(file_path: &str) -> PdfScanResult {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let scan_id = format!("scan_{}_{}",
        timestamp,
        Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .replace(' ', "_")
    );

    let pdfid_output = run_pdfid(file_path);
    let metadata = get_pdf_metadata(file_path);

    let result = match std::fs::read(file_path) {
        Ok(content) => {
            if content.windows(b"/OpenAction".len()).any(|window| window == b"/OpenAction") {
                PdfScanResult {
                    file_path: file_path.to_string(),
                    timestamp,
                    is_suspicious: true,
                    reason: "Found an /OpenAction tag.".to_string(),
                    scan_id,
                    pdfid_output,
                    metadata,
                }
            } else if content.windows(b"/AA".len()).any(|window| window == b"/AA") {
                PdfScanResult {
                    file_path: file_path.to_string(),
                    timestamp,
                    is_suspicious: true,
                    reason: "Found an /AA (Additional-Actions) tag.".to_string(),
                    scan_id,
                    pdfid_output,
                    metadata,
                }
            } else {
                PdfScanResult {
                    file_path: file_path.to_string(),
                    timestamp,
                    is_suspicious: false,
                    reason: "Clean.".to_string(),
                    scan_id,
                    pdfid_output,
                    metadata,
                }
            }
        }
        Err(e) => PdfScanResult {
            file_path: file_path.to_string(),
            timestamp,
            is_suspicious: true,
            reason: format!("Could not read file: {}", e),
            scan_id,
            pdfid_output,
            metadata,
        },
    };

    save_scan_result(&result);
    result
} 