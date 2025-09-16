use eframe::egui;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

pub struct BigmanApp {
    scan_path: String,
    scan_result: String,
    scanning: bool,
    scan_receiver: Option<mpsc::Receiver<String>>,
    clamscan_options: ClamScanOptions,
}

#[derive(Default)]
struct ClamScanOptions {
    recursive: bool,
    verbose: bool,
    quiet: bool,
    infected_only: bool,
    remove_infected: bool,
}

impl Default for BigmanApp {
    fn default() -> Self {
        Self {
            scan_path: "/home".to_string(),
            scan_result: String::new(),
            scanning: false,
            scan_receiver: None,
            clamscan_options: Default::default(),
        }
    }
}

impl eframe::App for BigmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(receiver) = &self.scan_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.scan_result = result;
                self.scanning = false;
                self.scan_receiver = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🛡️ BigMan Antivirus Scanner");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Scan path:");
                ui.text_edit_singleline(&mut self.scan_path);
                if ui.button("📁 Browse").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.scan_path = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.separator();

            ui.label("ClamScan Options:");
            ui.checkbox(&mut self.clamscan_options.recursive, "Recursive scan (-r)");
            ui.checkbox(&mut self.clamscan_options.verbose, "Verbose output (-v)");
            ui.checkbox(&mut self.clamscan_options.quiet, "Quiet mode (--quiet)");
            ui.checkbox(&mut self.clamscan_options.infected_only, "Show infected files only (-i)");
            ui.checkbox(&mut self.clamscan_options.remove_infected, "⚠️ Remove infected files (--remove)");

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("🔍 Start Scan").clicked() && !self.scanning {
                    self.start_scan();
                }

                if ui.button("🗑️ Clear Results").clicked() {
                    self.scan_result.clear();
                }

                if self.scanning {
                    ui.spinner();
                    ui.label("Scanning...");
                }
            });

            ui.separator();

            if !self.scan_result.is_empty() {
                ui.label("Scan Results:");
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.monospace(&self.scan_result);
                    });
            }
        });

        if self.scanning {
            ctx.request_repaint();
        }
    }
}

impl BigmanApp {
    fn start_scan(&mut self) {
        self.scanning = true;
        self.scan_result = "Starting scan...".to_string();

        let mut cmd = Command::new("clamscan");

        if self.clamscan_options.recursive {
            cmd.arg("-r");
        }
        if self.clamscan_options.verbose {
            cmd.arg("-v");
        }
        if self.clamscan_options.quiet {
            cmd.arg("--quiet");
        }
        if self.clamscan_options.infected_only {
            cmd.arg("-i");
        }
        if self.clamscan_options.remove_infected {
            cmd.arg("--remove");
        }

        cmd.arg(&self.scan_path);

        let (sender, receiver) = mpsc::channel();
        self.scan_receiver = Some(receiver);

        thread::spawn(move || {
            let output = cmd
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            let result = match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    let mut result = String::new();
                    if !stdout.is_empty() {
                        result.push_str(&stdout);
                    }
                    if !stderr.is_empty() {
                        if !result.is_empty() {
                            result.push_str("\n--- ERRORS ---\n");
                        }
                        result.push_str(&stderr);
                    }

                    if result.is_empty() {
                        result = "Scan completed with no output.".to_string();
                    }

                    match output.status.code() {
                        Some(0) => format!("✅ SCAN COMPLETE - No threats found\n\n{}", result),
                        Some(1) => format!("⚠️ THREATS DETECTED\n\n{}", result),
                        Some(code) => format!("❌ SCAN ERROR (exit code: {})\n\n{}", code, result),
                        None => format!("❌ SCAN TERMINATED\n\n{}", result),
                    }
                }
                Err(e) => format!("❌ Failed to run clamscan: {}\n\nMake sure ClamAV is installed:\nsudo apt install clamav clamav-daemon\nsudo freshclam", e),
            };

            let _ = sender.send(result);
        });
    }
}

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("BigMan Antivirus"),
        ..Default::default()
    };

    eframe::run_native(
        "BigMan Antivirus",
        options,
        Box::new(|_cc| Ok(Box::new(BigmanApp::default()))),
    )
}