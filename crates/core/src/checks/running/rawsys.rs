//! Reads that ask the kernel directly, without going through the C library.
//!
//! A library injected through `/etc/ld.so.preload` or `LD_PRELOAD` replaces the C
//! library's `open`, `read` and `readdir` inside every dynamically linked program,
//! Bigman included, and can leave things out of what they return. These functions
//! make the same system calls with the instruction itself, which such a library
//! cannot replace, so the two answers can be compared. A kernel module that
//! filters the system call defeats both.
use std::{ffi::CString, io, os::unix::ffi::OsStrExt, path::Path};

#[cfg(target_arch = "x86_64")]
unsafe fn syscall3(nr: i64, a: usize, b: usize, c: usize) -> isize {
    let ret: isize;
    core::arch::asm!(
        "syscall",
        inlateout("rax") nr as isize => ret,
        in("rdi") a,
        in("rsi") b,
        in("rdx") c,
        in("r10") 0usize,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    ret
}

#[cfg(target_arch = "aarch64")]
unsafe fn syscall3(nr: i64, a: usize, b: usize, c: usize) -> isize {
    let ret: isize;
    core::arch::asm!(
        "svc 0",
        in("x8") nr as usize,
        inlateout("x0") a => ret,
        in("x1") b,
        in("x2") c,
        in("x3") 0usize,
        options(nostack),
    );
    ret
}

/// Other architectures go through the C library's `syscall`, which a preloaded
/// library could replace; `DIRECT` says which one this build has.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
unsafe fn syscall3(nr: i64, a: usize, b: usize, c: usize) -> isize {
    let r = libc::syscall(nr as libc::c_long, a, b, c, 0usize);
    if r < 0 {
        -(*libc::__errno_location() as isize)
    } else {
        r as isize
    }
}

/// Whether these reads bypass the C library on this architecture.
pub const DIRECT: bool = cfg!(any(target_arch = "x86_64", target_arch = "aarch64"));

fn check(ret: isize) -> io::Result<usize> {
    if ret < 0 {
        Err(io::Error::from_raw_os_error(-ret as i32))
    } else {
        Ok(ret as usize)
    }
}

struct Fd(usize);

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe { syscall3(libc::SYS_close, self.0, 0, 0) };
    }
}

fn open(path: &Path, flags: i32) -> io::Result<Fd> {
    let c = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;
    // openat(AT_FDCWD, path, flags): the fourth argument (mode) is zero.
    let ret = unsafe {
        syscall3(
            libc::SYS_openat,
            libc::AT_FDCWD as usize,
            c.as_ptr() as usize,
            (flags | libc::O_CLOEXEC) as usize,
        )
    };
    check(ret).map(Fd)
}

/// Whether a process or thread with this ID exists: signal 0 delivers nothing
/// and only reports whether one could be sent. "Not permitted" is still "exists".
pub fn exists(pid: u32) -> bool {
    if pid == 0 || pid > i32::MAX as u32 {
        return false;
    }
    let ret = unsafe { syscall3(libc::SYS_kill, pid as usize, 0, 0) };
    ret == 0 || ret == -(libc::EPERM as isize)
}

/// A file's bytes, up to `max`. Files in /proc report no size, so this reads to the end.
pub fn read_file(path: &Path, max: usize) -> io::Result<Vec<u8>> {
    let fd = open(path, libc::O_RDONLY)?;
    let mut out = vec![];
    let mut buf = vec![0u8; 64 * 1024];
    while out.len() < max {
        let n =
            check(unsafe { syscall3(libc::SYS_read, fd.0, buf.as_mut_ptr() as usize, buf.len()) })?;
        if n == 0 {
            break;
        }
        out.extend_from_slice(&buf[..n]);
    }
    out.truncate(max);
    Ok(out)
}

/// The names in a directory, without "." and "..".
pub fn list_dir(path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let fd = open(path, libc::O_RDONLY | libc::O_DIRECTORY)?;
    let mut names = vec![];
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = check(unsafe {
            syscall3(
                libc::SYS_getdents64,
                fd.0,
                buf.as_mut_ptr() as usize,
                buf.len(),
            )
        })?;
        if n == 0 {
            break;
        }
        // struct linux_dirent64 { u64 d_ino; i64 d_off; u16 d_reclen; u8 d_type; char d_name[]; }
        let mut at = 0;
        while at + 19 <= n {
            let reclen = u16::from_ne_bytes([buf[at + 16], buf[at + 17]]) as usize;
            if reclen < 19 || at + reclen > n {
                break;
            }
            let name = &buf[at + 19..at + reclen];
            let name = &name[..name.iter().position(|b| *b == 0).unwrap_or(name.len())];
            if name != b"." && name != b".." {
                names.push(name.to_vec());
            }
            at += reclen;
        }
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn direct_reads_match_the_c_library_on_ordinary_files() {
        let d = tempfile::tempdir().unwrap();
        let big = "x".repeat(200_000);
        std::fs::write(d.path().join("a"), &big).unwrap();
        std::fs::write(d.path().join("b"), "two").unwrap();
        std::fs::create_dir(d.path().join("c")).unwrap();
        assert_eq!(
            read_file(&d.path().join("a"), usize::MAX).unwrap().len(),
            200_000
        );
        assert_eq!(read_file(&d.path().join("a"), 10).unwrap(), b"xxxxxxxxxx");
        assert_eq!(read_file(&d.path().join("b"), 1024).unwrap(), b"two");
        let listed: BTreeSet<Vec<u8>> = list_dir(d.path()).unwrap().into_iter().collect();
        let expected: BTreeSet<Vec<u8>> = [b"a".to_vec(), b"b".to_vec(), b"c".to_vec()].into();
        assert_eq!(listed, expected);
        let missing = read_file(&d.path().join("nope"), 10).unwrap_err();
        assert_eq!(missing.kind(), io::ErrorKind::NotFound);
        assert!(
            list_dir(&d.path().join("a")).is_err(),
            "a file is not a directory"
        );
    }

    #[test]
    fn a_directory_with_many_entries_is_listed_whole() {
        let d = tempfile::tempdir().unwrap();
        for i in 0..5_000 {
            std::fs::write(d.path().join(format!("entry-with-a-long-name-{i:05}")), "").unwrap();
        }
        assert_eq!(list_dir(d.path()).unwrap().len(), 5_000);
    }
}
