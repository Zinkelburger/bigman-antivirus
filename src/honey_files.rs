// src/honey_files.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use inotify::{Inotify, WatchMask};

/// Event types that can be monitored
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    Read,
    Write,
    Execute,
    Access,
    Modify,
    Create,
    Delete,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Read => "Read",
            EventType::Write => "Write",
            EventType::Execute => "Execute",
            EventType::Access => "Access",
            EventType::Modify => "Modify",
            EventType::Create => "Create",
            EventType::Delete => "Delete",
        }
    }

    pub fn all() -> Vec<EventType> {
        vec![
            EventType::Read,
            EventType::Write,
            EventType::Execute,
            EventType::Access,
            EventType::Modify,
            EventType::Create,
            EventType::Delete,
        ]
    }
}

/// Configuration for a honey file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyFileConfig {
    pub file_path: PathBuf,
    pub enabled: bool,
    pub monitored_events: Vec<EventType>,
    pub script_handler: Option<PathBuf>,
    pub description: String,
}

impl HoneyFileConfig {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            description: format!("Monitor {}", file_path.display()),
            file_path,
            enabled: true,
            monitored_events: vec![EventType::Write, EventType::Modify],
            script_handler: None,
        }
    }

    /// Get common sensitive files to monitor
    pub fn get_common_files() -> Vec<HoneyFileConfig> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let home_path = Path::new(&home);

        vec![
            HoneyFileConfig {
                file_path: home_path.join(".bashrc"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify],
                script_handler: None,
                description: "Bash configuration file".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".bash_profile"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify],
                script_handler: None,
                description: "Bash profile configuration".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".zshrc"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify],
                script_handler: None,
                description: "Zsh configuration file".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".gitconfig"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify, EventType::Access],
                script_handler: None,
                description: "Global Git configuration".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".git-credentials"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify, EventType::Access],
                script_handler: None,
                description: "Global Git configuration".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".ssh/config"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify, EventType::Access],
                script_handler: None,
                description: "SSH client configuration".to_string(),
            },
            HoneyFileConfig {
                file_path: home_path.join(".ssh/authorized_keys"),
                enabled: false,
                monitored_events: vec![EventType::Write, EventType::Modify, EventType::Access],
                script_handler: None,
                description: "SSH authorized keys".to_string(),
            },
        ]
    }
}

/// A file monitoring event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    pub file_path: PathBuf,
    pub event_type: EventType,
    pub timestamp: u64,
    pub process_info: Option<String>,
}

impl FileEvent {
    pub fn new(file_path: PathBuf, event_type: EventType) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            file_path,
            event_type,
            timestamp,
            process_info: None,
        }
    }
}

/// Honey file monitoring system
pub struct HoneyFileMonitor {
    configs: HashMap<PathBuf, HoneyFileConfig>,
    #[cfg(unix)]
    inotify_handle: Option<thread::JoinHandle<()>>,
}

impl HoneyFileMonitor {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            #[cfg(unix)]
            inotify_handle: None,
        }
    }

    pub fn add_file(&mut self, config: HoneyFileConfig) {
        self.configs.insert(config.file_path.clone(), config);
    }

    pub fn remove_file(&mut self, path: &Path) {
        self.configs.remove(path);
    }

    pub fn get_configs(&self) -> &HashMap<PathBuf, HoneyFileConfig> {
        &self.configs
    }

    pub fn get_configs_mut(&mut self) -> &mut HashMap<PathBuf, HoneyFileConfig> {
        &mut self.configs
    }

    #[cfg(unix)]
    pub fn start_monitoring(&mut self) -> mpsc::Receiver<FileEvent> {
        let (sender, receiver) = mpsc::channel();
        let configs: Vec<_> = self.configs.values()
            .filter(|c| c.enabled)
            .cloned()
            .collect();

        let handle = thread::spawn(move || {
            if let Err(e) = Self::monitor_files_unix(&configs, sender) {
                eprintln!("Honey file monitoring error: {}", e);
            }
        });

        self.inotify_handle = Some(handle);
        receiver
    }

    #[cfg(not(unix))]
    pub fn start_monitoring(&mut self) -> mpsc::Receiver<FileEvent> {
        let (_, receiver) = mpsc::channel();
        eprintln!("File monitoring not supported on this platform");
        receiver
    }

    #[cfg(unix)]
    fn monitor_files_unix(configs: &[HoneyFileConfig], sender: mpsc::Sender<FileEvent>) -> Result<(), Box<dyn std::error::Error>> {
        let mut inotify = Inotify::init()?;

        for config in configs {
            if !config.file_path.exists() {
                continue;
            }

            let mut mask = WatchMask::empty();
            for event_type in &config.monitored_events {
                mask |= match event_type {
                    EventType::Read | EventType::Access => WatchMask::ACCESS,
                    EventType::Write | EventType::Modify => WatchMask::MODIFY | WatchMask::CLOSE_WRITE,
                    EventType::Create => WatchMask::CREATE,
                    EventType::Delete => WatchMask::DELETE | WatchMask::DELETE_SELF,
                    EventType::Execute => WatchMask::ACCESS, // Best approximation
                };
            }

            if let Err(e) = inotify.watches().add(&config.file_path, mask) {
                eprintln!("Failed to watch {}: {}", config.file_path.display(), e);
            }
        }

        let mut buffer = [0; 1024];
        loop {
            let events = inotify.read_events_blocking(&mut buffer)?;

            for event in events {
                if let Some(name) = event.name {
                    let path = PathBuf::from(name.to_string_lossy().to_string());
                    let event_type = if event.mask.contains(inotify::EventMask::ACCESS) {
                        EventType::Access
                    } else if event.mask.contains(inotify::EventMask::MODIFY) || event.mask.contains(inotify::EventMask::CLOSE_WRITE) {
                        EventType::Modify
                    } else if event.mask.contains(inotify::EventMask::CREATE) {
                        EventType::Create
                    } else if event.mask.contains(inotify::EventMask::DELETE) {
                        EventType::Delete
                    } else {
                        EventType::Access
                    };

                    let file_event = FileEvent::new(path, event_type);

                    // Execute script handler if configured
                    if let Some(config) = configs.iter().find(|c| c.file_path.file_name() == file_event.file_path.file_name()) {
                        if let Some(ref script_path) = config.script_handler {
                            Self::execute_script_handler(script_path, &file_event);
                        }
                    }

                    if sender.send(file_event).is_err() {
                        break; // Channel closed
                    }
                }
            }
        }
    }

    fn execute_script_handler(script_path: &Path, event: &FileEvent) {
        use std::process::Command;

        if !script_path.exists() || !script_path.extension().map_or(false, |ext| ext == "sh") {
            return;
        }

        let mut cmd = Command::new("sh");
        cmd.arg(script_path)
           .env("HONEY_FILE_PATH", &event.file_path)
           .env("HONEY_EVENT_TYPE", event.event_type.as_str())
           .env("HONEY_TIMESTAMP", event.timestamp.to_string());

        if let Ok(output) = cmd.output() {
            if !output.status.success() {
                eprintln!("Script handler {} failed: {}",
                    script_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }
}

impl Default for HoneyFileMonitor {
    fn default() -> Self {
        Self::new()
    }
}