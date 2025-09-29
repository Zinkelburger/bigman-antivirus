use eframe::egui;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::mem;

// Import our new modules
use crate::pdf_scanner::PdfScanResult;
use crate::ipc::start_ipc_server;
use crate::honey_files::{HoneyFileMonitor, HoneyFileConfig, FileEvent, EventType};

/// Represents the state of a long-running task (scan or update).
enum Task {
    /// The task has not been started or has been cleared.
    Idle,
    /// The task is running in a background thread.
    InProgress(mpsc::Receiver<String>),
    /// The task has completed, and this is the result.
    Complete(String),
}

/// Options for the `clamscan` command.
#[derive(Default)]
struct ClamScanOptions {
    recursive: bool,
    verbose: bool,
    infected_only: bool,
    remove_infected: bool,
}

/// Enum to manage which view is active
#[derive(PartialEq)]
enum ActiveView {
    ClamAV,
    PdfScanner,
    HoneyFiles,
}

/// The main application state.
pub struct BigmanApp {
    scan_path: String,
    clamscan_options: ClamScanOptions,
    scan_task: Task,
    update_task: Task,
    // NEW state for the PDF scanner view
    active_view: ActiveView,
    pdf_scan_results: Vec<PdfScanResult>,
    ipc_receiver: Option<mpsc::Receiver<PdfScanResult>>,
    selected_scan_index: Option<usize>,
    // Honey file monitoring state
    honey_file_monitor: HoneyFileMonitor,
    honey_file_events: Vec<FileEvent>,
    honey_file_receiver: Option<mpsc::Receiver<FileEvent>>,
    selected_honey_file: Option<usize>,
    show_add_file_dialog: bool,
    new_file_path: String,
    new_file_description: String,
}

impl Default for BigmanApp {
    fn default() -> Self {
        Self {
            scan_path: "/home".to_string(),
            clamscan_options: ClamScanOptions::default(),
            scan_task: Task::Idle,
            // Start with a helpful message for the user.
            update_task: Task::Complete(
                "Database status is unknown. Click 'Update Database' to check for new definitions.".to_string(),
            ),
            // NEW default state
            active_view: ActiveView::ClamAV,
            pdf_scan_results: Vec::new(),
            ipc_receiver: None,
            selected_scan_index: None,
            // Honey file monitoring defaults
            honey_file_monitor: HoneyFileMonitor::new(),
            honey_file_events: Vec::new(),
            honey_file_receiver: None,
            selected_honey_file: None,
            show_add_file_dialog: false,
            new_file_path: String::new(),
            new_file_description: String::new(),
        }
    }
}

impl eframe::App for BigmanApp {
    /// Called each frame to update the GUI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for new PDF scan results from IPC
        if let Some(ref receiver) = self.ipc_receiver {
            while let Ok(result) = receiver.try_recv() {
                // Prepend to keep the latest result at the top
                self.pdf_scan_results.insert(0, result);
            }
        }

        // Check for new honey file events
        if let Some(ref receiver) = self.honey_file_receiver {
            while let Ok(event) = receiver.try_recv() {
                // Prepend to keep the latest event at the top
                self.honey_file_events.insert(0, event);
            }
        }

        // Check for results from any background tasks.
        self.check_for_task_completion();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_header_and_zoom(ui, ctx);
            ui.separator();
            
            // NEW: Draw UI based on the active view
            self.draw_view_switcher(ui);
            ui.separator();

            match self.active_view {
                ActiveView::ClamAV => {
                    self.draw_database_section(ui);
                    ui.separator();
                    self.draw_scan_section(ui);
                }
                ActiveView::PdfScanner => {
                    self.draw_pdf_scanner_view(ui);
                }
                ActiveView::HoneyFiles => {
                    self.draw_honey_files_view(ui);
                }
            }
        });

        // If a task is running, request a repaint to update the spinner.
        if matches!(self.scan_task, Task::InProgress(_)) || matches!(self.update_task, Task::InProgress(_)) {
            ctx.request_repaint();
        }
    }
}

impl BigmanApp {
    /// NEW: Draws the toggle buttons to switch between views.
    fn draw_view_switcher(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_view, ActiveView::ClamAV, "🛡️ ClamAV Scanner");
            ui.selectable_value(&mut self.active_view, ActiveView::PdfScanner, "📄 Live PDF Scans");
            ui.selectable_value(&mut self.active_view, ActiveView::HoneyFiles, "🍯 Honey Files");
        });
    }

    /// NEW: Draws the UI for displaying live PDF scan results.
    fn draw_pdf_scanner_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Live PDF Download Scanner");
        ui.label("This view automatically displays results for PDFs downloaded while the app is running.");

        ui.horizontal(|ui| {
            if ui.button("Clear Results").clicked() {
                self.pdf_scan_results.clear();
                self.selected_scan_index = None;
            }
            if ui.button("Load Previous Scans").clicked() {
                self.load_previous_scans();
            }
        });

        ui.separator();

        // Split view: List on left, details on right
        ui.columns(2, |columns| {
            // Left column: List of scans
            columns[0].label("Scan Results:");
            egui::ScrollArea::vertical().id_source("pdf_scan_list").show(&mut columns[0], |ui| {
                if self.pdf_scan_results.is_empty() {
                    ui.label("No PDFs scanned yet. Download a PDF to see results here.");
                } else {
                    for (idx, result) in self.pdf_scan_results.iter().enumerate() {
                        let color = if result.is_suspicious { egui::Color32::RED } else { egui::Color32::GREEN };

                        let is_selected = self.selected_scan_index == Some(idx);
                        let response = ui.selectable_label(is_selected, format!("{} - {}",
                            result.scan_id.chars().take(20).collect::<String>(),
                            result.reason
                        ));

                        if response.clicked() {
                            self.selected_scan_index = Some(idx);
                        }

                        ui.colored_label(color, &result.file_path);
                        ui.separator();
                    }
                }
            });

            // Right column: Scan details
            columns[1].label("Scan Details:");
            egui::ScrollArea::vertical().id_source("pdf_scan_details").show(&mut columns[1], |ui| {
                if let Some(idx) = self.selected_scan_index {
                    if let Some(result) = self.pdf_scan_results.get(idx) {
                        ui.heading("Scan Information");
                        ui.monospace(format!("Scan ID: {}", result.scan_id));
                        ui.monospace(format!("File: {}", result.file_path));
                        ui.monospace(format!("Timestamp: {}", result.timestamp));
                        ui.colored_label(
                            if result.is_suspicious { egui::Color32::RED } else { egui::Color32::GREEN },
                            format!("Status: {}", result.reason)
                        );

                        ui.separator();

                        if let Some(ref pdfid) = result.pdfid_output {
                            ui.collapsing("PDFiD Output", |ui| {
                                ui.monospace(pdfid);
                            });
                        }

                        if let Some(ref metadata) = result.metadata {
                            ui.collapsing("PDF Metadata", |ui| {
                                ui.monospace(metadata);
                            });
                        }
                    }
                } else {
                    ui.label("Click on a scan result to view details");
                }
            });
        });
    }

    fn load_previous_scans(&mut self) {
        use std::fs;
        use std::path::Path;

        let scan_dir = Path::new("pdf_scans");
        if scan_dir.exists() {
            if let Ok(entries) = fs::read_dir(scan_dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "json" {
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                if let Ok(result) = serde_json::from_str::<PdfScanResult>(&content) {
                                    // Check if not already in list
                                    if !self.pdf_scan_results.iter().any(|r| r.scan_id == result.scan_id) {
                                        self.pdf_scan_results.push(result);
                                    }
                                }
                            }
                        }
                    }
                }
                // Sort by timestamp, newest first
                self.pdf_scan_results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            }
        }
    }

    /// Draws the main header and zoom controls.
    fn draw_header_and_zoom(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("⚔ BigMan Antivirus Scanner");
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            if ui.button("-").clicked() {
                ctx.set_pixels_per_point((ctx.pixels_per_point() - 0.1).max(0.75));
            }
            ui.label(format!("{:.0}%", ctx.pixels_per_point() * 100.0));
            if ui.button("+").clicked() {
                ctx.set_pixels_per_point((ctx.pixels_per_point() + 0.1).min(3.0));
            }
            ui.label("(Use Ctrl +/- or Ctrl+Scroll)");
        });
    }

    /// Draws the UI for the virus database update section.
    fn draw_database_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("Virus Database");

        let is_task_running = !matches!(self.scan_task, Task::Idle | Task::Complete(_)) || !matches!(self.update_task, Task::Idle | Task::Complete(_));

        ui.horizontal(|ui| {
            let update_button = ui.add_enabled(!is_task_running, egui::Button::new("🔄 Update Database"));
            if update_button.clicked() {
                self.start_database_update();
            }

            if let Task::InProgress(_) = self.update_task {
                ui.spinner();
                ui.label("Updating...");
            }
        });

        if let Task::Complete(result) = &self.update_task {
            ui.add_space(5.0);
            ui.label("Last Update Result:");
            egui::ScrollArea::vertical().max_height(150.0).id_source("database_update_result").show(ui, |ui| {
                ui.monospace(result);
            });
            
            // Provide a helpful, non-intrusive tip for a very common configuration error.
            if result.contains("Can't open/parse the config file /etc/freshclam.conf") {
                ui.add_space(5.0);
                ui.colored_label(egui::Color32::YELLOW, "! Tip: This error often requires running `sudo freshclam` once to fix permissions, or commenting out the 'Example' line in /etc/freshclam.conf.");
            }
        }
    }
    
    /// Draws the UI for the main scanning configuration and results section.
    fn draw_scan_section(&mut self, ui: &mut egui::Ui) {
        ui.heading("File Scanner");
        
        let is_task_running = !matches!(self.scan_task, Task::Idle | Task::Complete(_)) || !matches!(self.update_task, Task::Idle | Task::Complete(_));

        // --- Scan Path ---
        ui.horizontal(|ui| {
            ui.label("Path to scan:");
            ui.add_enabled(!is_task_running, egui::TextEdit::singleline(&mut self.scan_path));
            if ui.add_enabled(!is_task_running, egui::Button::new("📁 Browse")).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.scan_path = path.to_string_lossy().to_string();
                }
            }
        });
        
        // --- Scan Options ---
        ui.label("Options:");
        ui.checkbox(&mut self.clamscan_options.recursive, "Recursive scan (-r)");
        ui.checkbox(&mut self.clamscan_options.infected_only, "Show infected files only (-i)");
        ui.checkbox(&mut self.clamscan_options.verbose, "Verbose output (-v)");
        ui.checkbox(&mut self.clamscan_options.remove_infected, "! Remove infected files (--remove)");

        ui.add_space(10.0);

        // --- Action Buttons ---
        ui.horizontal(|ui| {
            if ui.add_enabled(!is_task_running, egui::Button::new("🔍 Start Scan")).clicked() {
                self.start_scan();
            }
            if ui.button("🗑 Clear Results").clicked() {
                self.scan_task = Task::Idle;
            }

            if let Task::InProgress(_) = self.scan_task {
                ui.spinner();
                ui.label("Scanning...");
            }
        });

        // --- Scan Results ---
        if let Task::Complete(result) = &self.scan_task {
            ui.add_space(5.0);
            ui.separator();
            ui.label("Scan Results:");
            egui::ScrollArea::vertical().max_height(f32::INFINITY).id_source("scan_results").show(ui, |ui| {
                ui.monospace(result);
            });
        }
    }

    /// Kicks off a `clamscan` process in a background thread.
    fn start_scan(&mut self) {
        let mut cmd = Command::new("clamscan");

        if self.clamscan_options.recursive { cmd.arg("-r"); }
        if self.clamscan_options.verbose { cmd.arg("-v"); }
        if self.clamscan_options.infected_only { cmd.arg("-i"); }
        if self.clamscan_options.remove_infected { cmd.arg("--remove"); }

        cmd.arg(&self.scan_path);

        self.scan_task = Task::InProgress(run_command_in_thread(cmd, "clamscan"));
    }

    /// Kicks off a `freshclam` process in a background thread.
    fn start_database_update(&mut self) {
        let cmd = Command::new("freshclam");
        self.update_task = Task::InProgress(run_command_in_thread(cmd, "freshclam"));
    }

    /// Checks if any running tasks have finished and updates the state.
    fn check_for_task_completion(&mut self) {
        // This pattern uses `mem::replace` to temporarily take ownership of the task
        // so we can check the receiver, then puts the task back.
        let scan_task = mem::replace(&mut self.scan_task, Task::Idle);
        if let Task::InProgress(rx) = scan_task {
            match rx.try_recv() {
                Ok(result) => self.scan_task = Task::Complete(result),
                Err(mpsc::TryRecvError::Empty) => self.scan_task = Task::InProgress(rx), // Not done, put it back
                Err(mpsc::TryRecvError::Disconnected) => self.scan_task = Task::Complete("Task thread terminated unexpectedly.".to_string()),
            }
        } else {
            self.scan_task = scan_task; // Not in progress, put it back
        }

        let update_task = mem::replace(&mut self.update_task, Task::Idle);
        if let Task::InProgress(rx) = update_task {
            match rx.try_recv() {
                Ok(result) => self.update_task = Task::Complete(result),
                Err(mpsc::TryRecvError::Empty) => self.update_task = Task::InProgress(rx),
                Err(mpsc::TryRecvError::Disconnected) => self.update_task = Task::Complete("Task thread terminated unexpectedly.".to_string()),
            }
        } else {
            self.update_task = update_task;
        }
    }

    /// NEW: Draws the UI for honey file monitoring.
    fn draw_honey_files_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Honey File Monitor");
        ui.label("Monitor sensitive files for unauthorized access and modifications.");

        ui.horizontal(|ui| {
            if ui.button("Start Monitoring").clicked() && self.honey_file_receiver.is_none() {
                self.honey_file_receiver = Some(self.honey_file_monitor.start_monitoring());
            }

            if ui.button("Stop Monitoring").clicked() {
                self.honey_file_receiver = None;
            }

            if ui.button("Add File").clicked() {
                self.show_add_file_dialog = true;
                self.new_file_path.clear();
                self.new_file_description.clear();
            }

            if ui.button("Load Common Files").clicked() {
                let common_files = HoneyFileConfig::get_common_files();
                for config in common_files {
                    self.honey_file_monitor.add_file(config);
                }
            }

            if ui.button("Clear Events").clicked() {
                self.honey_file_events.clear();
            }
        });

        ui.separator();

        // Add file dialog
        if self.show_add_file_dialog {
            egui::Window::new("Add Honey File")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("File Path:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_file_path));
                        if ui.button("Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.new_file_path = path.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Description:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_file_description));
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Add").clicked() {
                            if !self.new_file_path.is_empty() {
                                let mut config = HoneyFileConfig::new(self.new_file_path.clone().into());
                                if !self.new_file_description.is_empty() {
                                    config.description = self.new_file_description.clone();
                                }
                                self.honey_file_monitor.add_file(config);
                                self.show_add_file_dialog = false;
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_add_file_dialog = false;
                        }
                    });
                });
        }

        // Split view: File configs on left, events on right
        ui.columns(2, |columns| {
            // Left column: File configurations
            columns[0].label("Monitored Files:");
            egui::ScrollArea::vertical().id_source("honey_file_configs").max_height(300.0).show(&mut columns[0], |ui| {
                let mut to_remove = Vec::new();
                let configs = self.honey_file_monitor.get_configs_mut();

                for (path, config) in configs.iter_mut() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut config.enabled, "");
                            ui.label(format!("{}", path.display()));
                            if ui.small_button("🗑").clicked() {
                                to_remove.push(path.clone());
                            }
                        });

                        ui.label(&config.description);

                        ui.horizontal(|ui| {
                            ui.label("Events:");
                            for event_type in EventType::all() {
                                let mut is_monitored = config.monitored_events.contains(&event_type);
                                if ui.checkbox(&mut is_monitored, event_type.as_str()).changed() {
                                    if is_monitored {
                                        if !config.monitored_events.contains(&event_type) {
                                            config.monitored_events.push(event_type);
                                        }
                                    } else {
                                        config.monitored_events.retain(|&e| e != event_type);
                                    }
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Script Handler:");
                            if let Some(ref script_path) = config.script_handler {
                                ui.label(format!("{}", script_path.display()));
                                if ui.small_button("Remove").clicked() {
                                    config.script_handler = None;
                                }
                            } else {
                                if ui.button("Set Script").clicked() {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .add_filter("Shell Script", &["sh"])
                                        .pick_file()
                                    {
                                        config.script_handler = Some(path);
                                    }
                                }
                            }
                        });
                    });
                    ui.separator();
                }

                // Remove files marked for deletion
                for path in to_remove {
                    self.honey_file_monitor.remove_file(&path);
                }
            });

            // Right column: Events
            columns[1].label("Recent Events:");
            egui::ScrollArea::vertical().id_source("honey_file_events").max_height(300.0).show(&mut columns[1], |ui| {
                if self.honey_file_events.is_empty() {
                    ui.label("No events recorded yet.");
                } else {
                    for (idx, event) in self.honey_file_events.iter().enumerate() {
                        let is_selected = self.selected_honey_file == Some(idx);
                        let response = ui.selectable_label(
                            is_selected,
                            format!("[{}] {} - {}",
                                event.event_type.as_str(),
                                event.file_path.display(),
                                chrono::DateTime::from_timestamp(event.timestamp as i64, 0)
                                    .map(|dt| dt.format("%H:%M:%S").to_string())
                                    .unwrap_or_else(|| "Unknown".to_string())
                            )
                        );

                        if response.clicked() {
                            self.selected_honey_file = Some(idx);
                        }

                        // Color code by event type
                        let color = match event.event_type {
                            EventType::Write | EventType::Modify => egui::Color32::RED,
                            EventType::Read | EventType::Access => egui::Color32::YELLOW,
                            EventType::Execute => egui::Color32::LIGHT_RED,
                            EventType::Create => egui::Color32::LIGHT_BLUE,
                            EventType::Delete => egui::Color32::DARK_RED,
                        };

                        ui.colored_label(color, format!("  {}", event.event_type.as_str()));
                        ui.separator();
                    }
                }
            });
        });

        // Event details panel
        if let Some(idx) = self.selected_honey_file {
            if let Some(event) = self.honey_file_events.get(idx) {
                ui.separator();
                ui.heading("Event Details");
                ui.monospace(format!("File: {}", event.file_path.display()));
                ui.monospace(format!("Event: {}", event.event_type.as_str()));
                ui.monospace(format!("Timestamp: {}", event.timestamp));
                if let Some(ref process_info) = event.process_info {
                    ui.monospace(format!("Process: {}", process_info));
                }
            }
        }
    }
}

/// A generic helper to run a `Command` in a background thread.
/// It returns a `Receiver` that will eventually contain the formatted output.
fn run_command_in_thread(mut command: Command, command_name: &'static str) -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let result_str = match command.stdout(Stdio::piped()).stderr(Stdio::piped()).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                let mut result = format!("Command finished with status: {}\n", output.status);
                if !stdout.is_empty() {
                    result.push_str("\n--- STDOUT ---\n");
                    result.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    result.push_str("\n--- STDERR ---\n");
                    result.push_str(&stderr);
                }
                result
            }
            Err(e) => format!(
                "❌ Failed to execute '{}': {}\n\nIs ClamAV installed and in your system's PATH?",
                command_name, e
            ),
        };
        // The receiver might be dropped if the app closes, so we ignore the send error.
        let _ = sender.send(result_str);
    });

    receiver
}

/// Entry point for the GUI application.
pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_title("BigMan Antivirus"),
        ..Default::default()
    };

    eframe::run_native(
        "BigMan Antivirus",
        options,
        Box::new(|_cc| {
            // Start the IPC server when the GUI is created
            let ipc_receiver = start_ipc_server();
            let mut app = BigmanApp::default();
            app.ipc_receiver = Some(ipc_receiver);
            Ok(Box::new(app))
        }),
    )
}