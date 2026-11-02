use crate::model::Report;
use anyhow::{bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};

pub const MAX_INPUT: u64 = 8 * 1024 * 1024;
#[derive(Clone)]
pub struct Store {
    pub root: PathBuf,
}
pub fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn read_file(path: &Path, max: u64) -> Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
        .with_context(|| format!("Cannot open {}", path.display()))?;
    let meta = file.metadata()?;
    if !meta.is_file() || meta.len() > max {
        bail!(
            "Expected a regular file no larger than {max} bytes: {}",
            path.display()
        );
    }
    let mut bytes = vec![];
    file.take(max + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > max {
        bail!("File grew beyond size limit");
    }
    Ok(bytes)
}
pub fn private_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)?;
    }
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() || meta.uid() != unsafe { libc::geteuid() } || meta.mode() & 0o077 != 0 {
        bail!(
            "Private directory must be owned by you, not a symlink, and mode 0700: {}",
            path.display()
        );
    }
    Ok(())
}
use std::os::unix::fs::DirBuilderExt;
impl Store {
    pub fn open(root: Option<PathBuf>) -> Result<Self> {
        let root = root.unwrap_or_else(|| {
            std::env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| home().join(".local/share"))
                .join("bigman")
        });
        if !root.is_absolute() {
            bail!("State directory must be absolute");
        }
        private_dir(&root)?;
        for name in [
            "reports",
            "summaries",
            "artifacts",
            "quarantine",
            "canaries",
        ] {
            private_dir(&root.join(name))?;
        }
        Ok(Self { root })
    }
    pub fn write<T: Serialize>(&self, name: &str, value: &T) -> Result<()> {
        self.bytes(name, &serde_json::to_vec_pretty(value)?)
    }
    pub fn bytes(&self, name: &str, bytes: &[u8]) -> Result<()> {
        let path = self.path(name)?;
        let mut tmp = tempfile::NamedTempFile::new_in(path.parent().context("Missing parent")?)?;
        tmp.as_file()
            .set_permissions(fs::Permissions::from_mode(0o600))?;
        tmp.write_all(bytes)?;
        tmp.as_file().sync_all()?;
        tmp.persist(&path).map_err(|e| e.error)?;
        Ok(())
    }
    pub fn path(&self, name: &str) -> Result<PathBuf> {
        if Path::new(name)
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            bail!("Invalid state path");
        }
        Ok(self.root.join(name))
    }
    pub fn read<T: DeserializeOwned>(&self, name: &str) -> Result<T> {
        Ok(serde_json::from_slice(&read_file(
            &self.path(name)?,
            32 * MAX_INPUT,
        )?)?)
    }
    pub fn lock(&self, name: &str) -> Result<File> {
        let f = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(self.path(name)?)?;
        fs2::FileExt::lock_exclusive(&f)?;
        Ok(f)
    }
    pub fn save(&self, r: &Report) -> Result<()> {
        self.write(&format!("reports/{}.json", r.id), r)?;
        let mut summary = r.clone();
        summary.details.clear();
        summary.coverage.notes = vec![format!(
            "History summary; {} findings and {} errors. Open the report for full evidence.",
            r.findings.len(),
            r.errors.len()
        )];
        summary.coverage.skipped.truncate(20);
        // Most severe first, so truncation never drops a High finding behind lesser ones.
        summary
            .findings
            .sort_by_key(|f| std::cmp::Reverse(f.severity));
        summary.findings.truncate(20);
        summary.summary = Some(crate::model::SummaryCounts {
            findings: r.findings.len() as u32,
            to_review: r
                .findings
                .iter()
                .filter(|f| f.severity != crate::model::Severity::Info)
                .count() as u32,
            errors: r.errors.len() as u32,
        });
        for finding in &mut summary.findings {
            finding.evidence = finding.evidence.chars().take(1000).collect();
        }
        summary.errors.truncate(20);
        for error in &mut summary.errors {
            *error = error.chars().take(1000).collect();
        }
        self.write(&format!("summaries/{}.json", r.id), &summary)
    }
    pub fn history(&self) -> Result<Vec<Report>> {
        let mut paths = Vec::new();
        for e in fs::read_dir(self.root.join("reports"))? {
            let e = e?;
            if e.path().extension().is_some_and(|x| x == "json") {
                paths.push((e.metadata()?.modified()?, e.path()));
            }
            if paths.len() > 100000 {
                bail!(
                    "History exceeds 100,000 records; archive older reports before loading history"
                );
            }
        }
        paths.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        paths.truncate(200);
        let mut reports = Vec::new();
        for (_, p) in paths {
            let summary = self
                .root
                .join("summaries")
                .join(p.file_name().context("Invalid report path")?);
            let path = if summary.exists() { summary } else { p };
            reports.push(serde_json::from_slice::<Report>(&read_file(
                &path,
                32 * MAX_INPUT,
            )?)?);
        }
        Ok(reports)
    }
    pub fn report(&self, id: &str) -> Result<Report> {
        uuid::Uuid::parse_str(id)?;
        self.read(&format!("reports/{id}.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_symlinks_and_special_files() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("link");
        std::os::unix::fs::symlink("/etc/passwd", &p).unwrap();
        assert!(read_file(&p, MAX_INPUT).is_err());
        assert!(read_file(Path::new("/dev/zero"), MAX_INPUT).is_err());
    }
    #[test]
    fn private_reports_roundtrip() {
        let d = tempfile::tempdir().unwrap();
        let s = Store::open(Some(d.path().join("state"))).unwrap();
        let r = Report::new("test", "fixture");
        s.save(&r).unwrap();
        assert_eq!(s.history().unwrap()[0].id, r.id);
        // Summaries keep the most severe findings and their true counts.
        let mut long = Report::new("Installer inspection", "fixture");
        for i in 0..25 {
            long.add(
                "shell.download",
                crate::model::Severity::Review,
                "Downloads",
                i,
                "",
            );
        }
        long.add(
            "shell.execute-download",
            crate::model::Severity::High,
            "Executes",
            26,
            "",
        );
        s.save(&long).unwrap();
        let summary = s
            .history()
            .unwrap()
            .into_iter()
            .find(|h| h.id == long.id)
            .unwrap();
        assert_eq!(summary.findings[0].rule, "shell.execute-download");
        assert_eq!(summary.findings.len(), 20);
        assert_eq!(summary.summary.unwrap().to_review, 26);
        assert!(s.report(&long.id).unwrap().summary.is_none());
        assert!(s.path("../escape").is_err());
        assert_eq!(
            fs::metadata(s.root.join("reports")).unwrap().mode() & 0o777,
            0o700
        );
    }
}
