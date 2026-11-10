//! What /proc says about the process behind an access. Read as soon as the
//! event arrives; a process that already exited leaves only its PID.
use crate::Reader;
use std::{fs, io::Read, path::Path};

const CMDLINE_MAX: u64 = 4096;
const PARENTS_MAX: usize = 6;

fn read_small(path: &Path, max: u64) -> Option<Vec<u8>> {
    let mut out = vec![];
    fs::File::open(path)
        .ok()?
        .take(max)
        .read_to_end(&mut out)
        .ok()?;
    Some(out)
}

fn status_field<'a>(status: &'a str, key: &str) -> Option<&'a str> {
    status
        .lines()
        .find_map(|l| l.strip_prefix(key)?.strip_prefix(':'))
        .map(str::trim)
}

/// The start time (field 22) from /proc/PID/stat. The name field may hold
/// spaces and parentheses, so fields are counted after its last ')'.
pub fn start_time(proc: &Path, pid: u32) -> Option<u64> {
    let stat = read_small(&proc.join(pid.to_string()).join("stat"), 4096)?;
    let stat = String::from_utf8_lossy(&stat);
    let rest = &stat[stat.rfind(')')? + 1..];
    rest.split_whitespace().nth(19)?.parse().ok()
}

/// The user name for a UID from /etc/passwd, else from the name service.
pub fn user_name(uid: u32) -> Option<String> {
    let mut buf = vec![0 as libc::c_char; 16 * 1024];
    let mut pwd: libc::passwd = unsafe { std::mem::zeroed() };
    let mut out = std::ptr::null_mut();
    let r = unsafe { libc::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut out) };
    if r != 0 || out.is_null() {
        return None;
    }
    Some(
        unsafe { std::ffi::CStr::from_ptr(pwd.pw_name) }
            .to_string_lossy()
            .into_owned(),
    )
}

/// Describes `pid`. `show_exe_for` is the watching user: the executable path is
/// only included for that user's own processes.
pub fn describe(proc: &Path, pid: u32, show_exe_for: u32) -> Reader {
    let dir = proc.join(pid.to_string());
    let status = read_small(&dir.join("status"), 16 * 1024)
        .map(|s| String::from_utf8_lossy(&s).into_owned())
        .unwrap_or_default();
    let uid = status_field(&status, "Uid")
        .and_then(|v| v.split_whitespace().next())
        .and_then(|v| v.parse().ok());
    let login_uid = read_small(&dir.join("loginuid"), 32)
        .and_then(|s| String::from_utf8_lossy(&s).trim().parse::<u32>().ok())
        .filter(|u| *u != u32::MAX);
    let cmdline = read_small(&dir.join("cmdline"), CMDLINE_MAX)
        .map(|c| {
            c.split(|b| *b == 0)
                .filter(|a| !a.is_empty())
                .map(|a| String::from_utf8_lossy(a).into_owned())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    let exe = (uid == Some(show_exe_for))
        .then(|| fs::read_link(dir.join("exe")).ok())
        .flatten()
        .map(|p| p.to_string_lossy().into_owned());
    let mut parents = vec![];
    let mut parent: Option<u32> = status_field(&status, "PPid").and_then(|p| p.parse().ok());
    while let Some(ppid) = parent.filter(|p| *p > 0 && parents.len() < PARENTS_MAX) {
        let s = read_small(&proc.join(ppid.to_string()).join("status"), 16 * 1024)
            .map(|s| String::from_utf8_lossy(&s).into_owned())
            .unwrap_or_default();
        let name = status_field(&s, "Name").unwrap_or("?");
        parents.push(format!("{name} ({ppid})"));
        parent = status_field(&s, "PPid").and_then(|p| p.parse().ok());
    }
    Reader {
        pid,
        uid,
        user: uid.and_then(user_name),
        login_uid,
        name: status_field(&status, "Name")
            .unwrap_or_default()
            .to_string(),
        exe,
        cmdline,
        start_time: start_time(proc, pid),
        parents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_this_process() {
        let me = unsafe { libc::getuid() };
        let r = describe(Path::new("/proc"), std::process::id(), me);
        assert_eq!(r.uid, Some(me));
        assert!(r.exe.is_some());
        assert!(!r.cmdline.is_empty());
        assert!(r.start_time.is_some());
        assert!(!r.parents.is_empty());
        let hidden = describe(Path::new("/proc"), std::process::id(), me + 1);
        assert!(
            hidden.exe.is_none(),
            "another user's view must not show the exe"
        );
    }

    #[test]
    fn reads_the_start_time_past_a_name_with_parentheses() {
        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir(d.path().join("7")).unwrap();
        let fields: Vec<String> = (3..=22).map(|n| n.to_string()).collect();
        std::fs::write(
            d.path().join("7/stat"),
            format!("7 (a) b) (c) {}\n", fields.join(" ")),
        )
        .unwrap();
        assert_eq!(start_time(d.path(), 7), Some(22));
    }

    #[test]
    fn a_vanished_process_leaves_only_its_pid() {
        let d = tempfile::tempdir().unwrap();
        let r = describe(d.path(), 99, 0);
        assert_eq!(
            r,
            Reader {
                pid: 99,
                ..Reader::default()
            }
        );
    }
}
