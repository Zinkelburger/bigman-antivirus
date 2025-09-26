// src/pdf_scanner.rs

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// The result of a single PDF scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfScanResult {
    pub file_path: String,
    pub timestamp: u64,
    pub is_suspicious: bool,
    pub reason: String,
}

/// Scans a PDF file for suspicious auto-action tags.
pub fn scan_pdf_for_actions(file_path: &str) -> PdfScanResult {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    match std::fs::read(file_path) {
        Ok(content) => {
            if content.windows(b"/OpenAction".len()).any(|window| window == b"/OpenAction") {
                return PdfScanResult {
                    file_path: file_path.to_string(),
                    timestamp,
                    is_suspicious: true,
                    reason: "Found an /OpenAction tag.".to_string(),
                };
            }
            if content.windows(b"/AA".len()).any(|window| window == b"/AA") {
                return PdfScanResult {
                    file_path: file_path.to_string(),
                    timestamp,
                    is_suspicious: true,
                    reason: "Found an /AA (Additional-Actions) tag.".to_string(),
                };
            }
            
            PdfScanResult {
                file_path: file_path.to_string(),
                timestamp,
                is_suspicious: false,
                reason: "Clean.".to_string(),
            }
        }
        Err(e) => PdfScanResult {
            file_path: file_path.to_string(),
            timestamp,
            is_suspicious: true,
            reason: format!("Could not read file: {}", e),
        },
    }
} 