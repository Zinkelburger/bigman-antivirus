// src/ipc.rs

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use crate::pdf_scanner::{scan_pdf_for_actions, PdfScanResult};

const IPC_ADDRESS: &str = "127.0.0.1:56789"; // An unused port for local communication

/// Starts the IPC server in a background thread to listen for scan requests.
/// Returns a receiver that the GUI can use to get scan results.
pub fn start_ipc_server() -> mpsc::Receiver<PdfScanResult> {
    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || {
        let listener = match TcpListener::bind(IPC_ADDRESS) {
            Ok(l) => l,
            Err(_) => {
                // Another instance might be running. For this example, we just exit the thread.
                return;
            }
        };

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                if let Ok(size) = stream.read(&mut buffer) {
                    let file_path = String::from_utf8_lossy(&buffer[..size]).to_string();
                    let result = scan_pdf_for_actions(&file_path);
                    
                    // Send the result back to the GUI thread
                    let _ = sender.send(result);
                }
            }
        }
    });

    receiver
}

/// Called by the native messaging host to send a file path to the running GUI server.
pub fn send_path_to_gui(file_path: &str) -> Result<(), std::io::Error> {
    match TcpStream::connect(IPC_ADDRESS) {
        Ok(mut stream) => {
            stream.write_all(file_path.as_bytes())?;
            stream.flush()
        }
        Err(e) => {
            // This error means the GUI is not running, which is okay.
            Err(e)
        }
    }
} 