// src/main.rs

// Declare our modules
mod analyzer;
mod gui;
mod pdf_scanner;
mod ipc;
mod honey_files;

use std::io::{self, Read};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

// Import structs and functions from our new files
use ipc::send_path_to_gui;

// Native messaging structs
#[derive(Deserialize, Debug)]
struct ExtensionMessage { 
    #[serde(rename = "filePath")]
    file_path: String, 
}

#[derive(Serialize, Debug)]
struct ScanResponse { 
    status: String, 
    reason: String, 
}

fn main() -> Result<()> {
    // Check if we have command line arguments (native messaging mode)
    if std::env::args().len() > 1 {
        run_native_messaging_host();
        return Ok(());
    }

    // Check if we're receiving piped input
    let is_piped = !is_tty();

    if is_piped {
        analyze_stdin()
    } else {
        gui::run_gui().map_err(|e| anyhow::anyhow!("GUI error: {}", e))
    }
}

fn analyze_stdin() -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)
        .context("Failed to read from stdin")?;

    if content.trim().is_empty() {
        return Ok(());
    }

    let analysis_result = analyzer::analyze_content(&content)?;

    if analysis_result.is_safe {
        println!("SAFE");
        std::process::exit(0);
    } else {
        for threat in &analysis_result.threats {
            eprintln!("THREAT: {}", threat);
        }
        std::process::exit(1);
    }
}

/// Native messaging host that scans PDFs and responds to the browser extension
fn run_native_messaging_host() {
    // Chrome Native Messaging protocol: Read 4-byte length prefix first
    let mut length_bytes = [0u8; 4];
    if std::io::stdin().read_exact(&mut length_bytes).is_err() {
        return;
    }
    let message_length = u32::from_le_bytes(length_bytes) as usize;

    // Read the JSON message
    let mut buffer = vec![0u8; message_length];
    if std::io::stdin().read_exact(&mut buffer).is_err() {
        return;
    }

    let input: ExtensionMessage = match serde_json::from_slice(&buffer) {
        Ok(msg) => msg,
        Err(_) => return, // Invalid input, exit silently
    };

    // First, try to send the path to the running GUI (if any)
    let _ = send_path_to_gui(&input.file_path);

    // Scan the PDF and send response back to browser extension
    let scan_result = pdf_scanner::scan_pdf_for_actions(&input.file_path);
    
    let response = if scan_result.is_suspicious {
        ScanResponse {
            status: "SUSPICIOUS".to_string(),
            reason: scan_result.reason.clone(),
        }
    } else {
        ScanResponse {
            status: "CLEAN".to_string(),
            reason: scan_result.reason.clone(),
        }
    };

    // Send response back to browser extension via stdout
    if let Ok(json_response) = serde_json::to_string(&response) {
        let message_length = json_response.len() as u32;
        
        // Native messaging protocol: 4 bytes for length, then JSON
        let _ = std::io::Write::write_all(&mut std::io::stdout(), &message_length.to_le_bytes());
        let _ = std::io::Write::write_all(&mut std::io::stdout(), json_response.as_bytes());
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}

fn is_tty() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
} 