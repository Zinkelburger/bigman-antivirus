use eframe::egui;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::mem;

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

/// The main application state.
pub struct BigmanApp {
    scan_path: String,
    clamscan_options: ClamScanOptions,
    scan_task: Task,
    update_task: Task,
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
        }
    }
}

impl eframe::App for BigmanApp {
    /// Called each frame to update the GUI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for results from any background tasks.
        self.check_for_task_completion();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_header_and_zoom(ui, ctx);
            ui.separator();
            self.draw_database_section(ui);
            ui.separator();
            self.draw_scan_section(ui);
        });

        // If a task is running, request a repaint to update the spinner.
        if matches!(self.scan_task, Task::InProgress(_)) || matches!(self.update_task, Task::InProgress(_)) {
            ctx.request_repaint();
        }
    }
}

impl BigmanApp {
    /// Draws the main header and zoom controls.
    fn draw_header_and_zoom(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("🛡️ BigMan Antivirus Scanner");
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            if ui.button("➖").clicked() {
                ctx.set_pixels_per_point((ctx.pixels_per_point() - 0.1).max(0.75));
            }
            ui.label(format!("{:.0}%", ctx.pixels_per_point() * 100.0));
            if ui.button("➕").clicked() {
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
            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                ui.monospace(result);
            });
            
            // Provide a helpful, non-intrusive tip for a very common configuration error.
            if result.contains("Can't open/parse the config file /etc/freshclam.conf") {
                ui.add_space(5.0);
                ui.colored_label(egui::Color32::YELLOW, "💡 Tip: This error often requires running `sudo freshclam` once to fix permissions, or commenting out the 'Example' line in /etc/freshclam.conf.");
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
        ui.checkbox(&mut self.clamscan_options.remove_infected, "⚠️ Remove infected files (--remove)");

        ui.add_space(10.0);

        // --- Action Buttons ---
        ui.horizontal(|ui| {
            if ui.add_enabled(!is_task_running, egui::Button::new("🔍 Start Scan")).clicked() {
                self.start_scan();
            }
            if ui.button("🗑️ Clear Results").clicked() {
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
            egui::ScrollArea::vertical().max_height(f32::INFINITY).show(ui, |ui| {
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
        Box::new(|_cc| Ok(Box::new(BigmanApp::default()))),
    )
}