//! Choices that are not notifications: which of the daemon's automatic checks run.
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The automatic checks you can turn off. Everything else the daemon collects
/// only reads what is already on this computer, and is listed in Setup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(default)]
#[ts(export)]
pub struct Preferences {
    /// Check for security updates once a day and after package changes. This uses
    /// the network for public data only; no package list is sent.
    pub check_updates: bool,
    /// Check each new file in Downloads as it arrives.
    pub watch_downloads: bool,
    /// Check each new project folder as it is cloned or unpacked.
    pub watch_projects: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            check_updates: true,
            watch_downloads: true,
            watch_projects: true,
        }
    }
}

impl Preferences {
    pub const KEY: &'static str = "preferences";

    pub fn load(db: &crate::store::db::Db) -> Preferences {
        db.setting(Self::KEY)
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_default()
    }

    /// Whether the source with this name may run.
    pub fn allows(&self, source: &str) -> bool {
        match source {
            "updates" => self.check_updates,
            "downloads" => self.watch_downloads,
            "projects" => self.watch_projects,
            _ => true,
        }
    }
}
