//! Per-connection TCP byte counters from the kernel's socket diagnostics
//! (`NETLINK_SOCK_DIAG`, what `ss -ti` reads). No privileges are needed.
use super::{linux::own_socket_owners, ProcessInfo, SocketCounters, TcpCounters, TcpTable};
use anyhow::{bail, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    sync::Mutex,
};

const NETLINK_SOCK_DIAG: libc::c_int = 4;
const SOCK_DIAG_BY_FAMILY: u16 = 20;
const NLM_F_REQUEST: u16 = 0x1;
const NLM_F_DUMP: u16 = 0x300;
const NLMSG_ERROR: u16 = 2;
const NLMSG_DONE: u16 = 3;
const NLMSG_HEADER: usize = 16;
/// `struct inet_diag_msg`.
const DIAG_MSG: usize = 72;
/// The attribute carrying `struct tcp_info`, and its bit in the request.
const INET_DIAG_INFO: u16 = 2;
/// Offsets of `tcpi_bytes_acked` and `tcpi_bytes_received` in `struct tcp_info`
/// (kernel 4.1 and later; they are part of the stable user ABI).
const BYTES_ACKED: usize = 120;
const BYTES_RECEIVED: usize = 128;
const TCP_CLOSE: u8 = 7;
const TCP_LISTEN: u8 = 10;

/// One socket out of a dump, before it is matched to a program.
#[derive(Debug, Clone, PartialEq)]
pub struct DiagSocket {
    pub state: u8,
    pub cookie: u64,
    pub local: SocketAddr,
    pub remote: SocketAddr,
    pub uid: u32,
    pub inode: u64,
    /// Bytes acknowledged and received; absent for sockets without TCP state.
    pub bytes: Option<(u64, u64)>,
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_ne_bytes([b[at], b[at + 1]])
}
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(b[at..at + 4].try_into().expect("four bytes"))
}
fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_ne_bytes(b[at..at + 8].try_into().expect("eight bytes"))
}

fn address(family: u8, raw: &[u8], port: u16) -> Option<SocketAddr> {
    let ip = match i32::from(family) {
        libc::AF_INET => IpAddr::V4(Ipv4Addr::new(raw[0], raw[1], raw[2], raw[3])),
        libc::AF_INET6 => {
            let v6 = Ipv6Addr::from(<[u8; 16]>::try_from(&raw[..16]).ok()?);
            // A dual-stack socket talking IPv4 is that IPv4 address.
            v6.to_ipv4_mapped().map_or(IpAddr::V6(v6), IpAddr::V4)
        }
        _ => return None,
    };
    Some(SocketAddr::new(ip, port))
}

/// Sockets out of one or more netlink messages. Returns whether the dump ended.
pub fn parse_inet_diag(mut buf: &[u8], out: &mut Vec<DiagSocket>) -> Result<bool> {
    while buf.len() >= NLMSG_HEADER {
        let len = u32_at(buf, 0) as usize;
        if len < NLMSG_HEADER || len > buf.len() {
            bail!("A socket-diagnostics message is truncated");
        }
        let (message, kind) = (&buf[..len], u16_at(buf, 4));
        // Messages are padded to four bytes.
        buf = &buf[((len + 3) & !3).min(buf.len())..];
        match kind {
            NLMSG_DONE => return Ok(true),
            NLMSG_ERROR => {
                let code = message
                    .get(NLMSG_HEADER..NLMSG_HEADER + 4)
                    .map_or(0, |c| i32::from_ne_bytes(c.try_into().expect("four bytes")));
                bail!(
                    "The kernel refused the socket-diagnostics request: {}",
                    std::io::Error::from_raw_os_error(-code)
                );
            }
            SOCK_DIAG_BY_FAMILY => {}
            _ => continue,
        }
        let Some(m) = message.get(NLMSG_HEADER..).filter(|m| m.len() >= DIAG_MSG) else {
            continue;
        };
        let (family, state) = (m[0], m[1]);
        let port = |at: usize| u16::from_be_bytes([m[at], m[at + 1]]);
        let (Some(local), Some(remote)) = (
            address(family, &m[8..24], port(4)),
            address(family, &m[24..40], port(6)),
        ) else {
            continue;
        };
        let mut bytes = None;
        let mut attrs = &m[DIAG_MSG..];
        while attrs.len() >= 4 {
            let alen = usize::from(u16_at(attrs, 0));
            if alen < 4 || alen > attrs.len() {
                break;
            }
            let payload = &attrs[4..alen];
            if u16_at(attrs, 2) == INET_DIAG_INFO && payload.len() >= BYTES_RECEIVED + 8 {
                bytes = Some((
                    u64_at(payload, BYTES_ACKED),
                    u64_at(payload, BYTES_RECEIVED),
                ));
            }
            attrs = &attrs[((alen + 3) & !3).min(attrs.len())..];
        }
        out.push(DiagSocket {
            state,
            cookie: u64::from(u32_at(m, 44)) | (u64::from(u32_at(m, 48)) << 32),
            local,
            remote,
            uid: u32_at(m, 64),
            inode: u64::from(u32_at(m, 68)),
            bytes,
        });
    }
    Ok(false)
}

/// Every TCP socket of one address family, in every state.
fn dump(family: libc::c_int) -> Result<Vec<DiagSocket>> {
    let fd = unsafe {
        libc::socket(
            libc::AF_NETLINK,
            libc::SOCK_DGRAM | libc::SOCK_CLOEXEC,
            NETLINK_SOCK_DIAG,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error()).context("Cannot open a netlink socket");
    }
    let fd = unsafe { OwnedFd::from_raw_fd(fd) };
    // A kernel that never answers must not hold the collector's thread.
    let wait = libc::timeval {
        tv_sec: 2,
        tv_usec: 0,
    };
    unsafe {
        libc::setsockopt(
            fd.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            std::ptr::from_ref(&wait).cast(),
            size_of::<libc::timeval>() as libc::socklen_t,
        );
    }
    // nlmsghdr, then inet_diag_req_v2: family, protocol, extensions, pad, states, socket id.
    let mut request = [0u8; NLMSG_HEADER + 56];
    let length = request.len() as u32;
    request[0..4].copy_from_slice(&length.to_ne_bytes());
    request[4..6].copy_from_slice(&SOCK_DIAG_BY_FAMILY.to_ne_bytes());
    request[6..8].copy_from_slice(&(NLM_F_REQUEST | NLM_F_DUMP).to_ne_bytes());
    request[8..12].copy_from_slice(&1u32.to_ne_bytes());
    request[16] = family as u8;
    request[17] = libc::IPPROTO_TCP as u8;
    request[18] = 1 << (INET_DIAG_INFO - 1);
    request[20..24].copy_from_slice(&u32::MAX.to_ne_bytes());
    let sent = unsafe { libc::send(fd.as_raw_fd(), request.as_ptr().cast(), request.len(), 0) };
    if sent < 0 {
        return Err(std::io::Error::last_os_error()).context("Cannot ask for socket diagnostics");
    }
    let mut out = vec![];
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = unsafe { libc::recv(fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len(), 0) };
        if n < 0 {
            let e = std::io::Error::last_os_error();
            if e.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(e).context("Cannot read socket diagnostics");
        }
        if n == 0 || parse_inet_diag(&buf[..n as usize], &mut out)? {
            return Ok(out);
        }
    }
}

/// The kernel's TCP table with each of your own sockets matched to its program.
/// The match walks `/proc/*/fd`, so it is repeated only when a socket appears
/// that the last walk did not explain.
#[derive(Default)]
pub struct SockDiag {
    owners: Mutex<Owners>,
}

#[derive(Default)]
struct Owners {
    by_inode: HashMap<u64, ProcessInfo>,
    /// The program each socket was last matched to, by socket cookie. A socket a
    /// program has closed stays in the table while its last bytes are
    /// acknowledged, with no file descriptor left to find it by.
    by_socket: HashMap<u64, ProcessInfo>,
    /// Your sockets the last walk found in no process: held open by the kernel
    /// while they close, or by a process /proc does not show.
    unexplained: HashSet<u64>,
}

impl SocketCounters for SockDiag {
    fn tcp(&self) -> Result<TcpTable> {
        let mut raw = dump(libc::AF_INET)?;
        // A machine with IPv6 turned off has no second table.
        raw.extend(dump(libc::AF_INET6).unwrap_or_default());
        let uid = unsafe { libc::getuid() };
        let mine: HashSet<u64> = raw
            .iter()
            .filter(|s| s.uid == uid && s.inode != 0 && s.bytes.is_some())
            .map(|s| s.inode)
            .collect();
        let mut owners = self.owners.lock().unwrap_or_else(|p| p.into_inner());
        if mine
            .iter()
            .any(|i| !owners.by_inode.contains_key(i) && !owners.unexplained.contains(i))
        {
            owners.by_inode = own_socket_owners();
            owners.unexplained = mine
                .iter()
                .filter(|i| !owners.by_inode.contains_key(i))
                .copied()
                .collect();
        }
        owners.by_inode.retain(|inode, _| mine.contains(inode));
        owners.unexplained.retain(|inode| mine.contains(inode));
        let mut seen = HashSet::with_capacity(raw.len());
        let mut table = TcpTable::default();
        for s in raw {
            if s.state == TCP_LISTEN {
                table.listening.push(s.local.port());
                continue;
            }
            let (Some((sent, received)), false) = (s.bytes, s.state == TCP_CLOSE) else {
                continue;
            };
            seen.insert(s.cookie);
            let process = match owners.by_inode.get(&s.inode).cloned() {
                Some(p) => {
                    owners.by_socket.insert(s.cookie, p.clone());
                    Some(p)
                }
                None => owners.by_socket.get(&s.cookie).cloned(),
            };
            table.sockets.push(TcpCounters {
                id: s.cookie,
                local: s.local,
                remote: s.remote,
                uid: s.uid,
                sent,
                received,
                process,
            });
        }
        owners.by_socket.retain(|cookie, _| seen.contains(cookie));
        table.listening.sort_unstable();
        table.listening.dedup();
        Ok(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    /// One netlink message holding an IPv4 socket and its `tcp_info`.
    fn message(state: u8, sport: u16, dport: u16, acked: u64, received: u64) -> Vec<u8> {
        let mut info = vec![0u8; 232];
        info[BYTES_ACKED..BYTES_ACKED + 8].copy_from_slice(&acked.to_ne_bytes());
        info[BYTES_RECEIVED..BYTES_RECEIVED + 8].copy_from_slice(&received.to_ne_bytes());
        let mut m = vec![0u8; DIAG_MSG];
        m[0] = libc::AF_INET as u8;
        m[1] = state;
        m[4..6].copy_from_slice(&sport.to_be_bytes());
        m[6..8].copy_from_slice(&dport.to_be_bytes());
        m[8..12].copy_from_slice(&[10, 0, 0, 5]);
        m[24..28].copy_from_slice(&[93, 184, 215, 14]);
        m[44..48].copy_from_slice(&7u32.to_ne_bytes());
        m[48..52].copy_from_slice(&1u32.to_ne_bytes());
        m[64..68].copy_from_slice(&1000u32.to_ne_bytes());
        m[68..72].copy_from_slice(&4242u32.to_ne_bytes());
        // An attribute the parser skips (odd length, so it is padded), then the info.
        m.extend([5, 0, 9, 0, 1, 0, 0, 0]);
        m.extend(((info.len() + 4) as u16).to_ne_bytes());
        m.extend(INET_DIAG_INFO.to_ne_bytes());
        m.extend(info);
        let mut out = ((NLMSG_HEADER + m.len()) as u32).to_ne_bytes().to_vec();
        out.extend(SOCK_DIAG_BY_FAMILY.to_ne_bytes());
        out.extend([0u8; 10]);
        out.extend(m);
        out
    }

    #[test]
    fn reads_sockets_and_their_byte_counters_from_a_dump() {
        let mut buf = message(1, 44372, 443, 9_717, 15_819);
        buf.extend(message(TCP_LISTEN, 8080, 0, 0, 0));
        let mut done = (NLMSG_HEADER as u32).to_ne_bytes().to_vec();
        done.extend(NLMSG_DONE.to_ne_bytes());
        done.extend([0u8; 10]);
        buf.extend(done);
        let mut out = vec![];
        assert!(parse_inet_diag(&buf, &mut out).unwrap(), "the dump ended");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].local.to_string(), "10.0.0.5:44372");
        assert_eq!(out[0].remote.to_string(), "93.184.215.14:443");
        assert_eq!(out[0].bytes, Some((9_717, 15_819)));
        assert_eq!((out[0].uid, out[0].inode), (1000, 4242));
        assert_eq!(out[0].cookie, (1 << 32) | 7);
        assert_eq!(out[1].state, TCP_LISTEN);
    }

    #[test]
    fn a_kernel_error_and_a_cut_message_are_errors_not_empty_tables() {
        let mut error = ((NLMSG_HEADER + 4) as u32).to_ne_bytes().to_vec();
        error.extend(NLMSG_ERROR.to_ne_bytes());
        error.extend([0u8; 10]);
        error.extend((-libc::EPERM).to_ne_bytes());
        let message = parse_inet_diag(&error, &mut vec![])
            .unwrap_err()
            .to_string();
        assert!(message.contains("refused"), "{message}");
        let whole = message_bytes();
        assert!(parse_inet_diag(&whole[..whole.len() - 8], &mut vec![]).is_err());
    }

    fn message_bytes() -> Vec<u8> {
        message(1, 1, 2, 3, 4)
    }

    #[test]
    fn counts_the_bytes_of_a_real_connection_and_names_this_process() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let mut client = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        let (mut server, _) = listener.accept().unwrap();
        client.write_all(&[7u8; 5_000]).unwrap();
        let mut got = vec![0u8; 5_000];
        server.read_exact(&mut got).unwrap();
        let table = match SockDiag::default().tcp() {
            Ok(t) => t,
            // Some build sandboxes forbid netlink; the parser tests still run there.
            Err(e) => return eprintln!("socket diagnostics unavailable here: {e:#}"),
        };
        assert!(table.listening.contains(&port));
        let ours = |s: &&TcpCounters| s.remote.port() == port && s.remote.ip().is_loopback();
        let sending = table.sockets.iter().find(ours).expect("the client socket");
        assert!(sending.sent >= 5_000, "sent {}", sending.sent);
        assert_eq!(
            sending.process.as_ref().map(|p| p.pid),
            Some(std::process::id())
        );
        let receiving = table
            .sockets
            .iter()
            .find(|s| s.local.port() == port && s.remote == sending.local)
            .expect("the accepted socket");
        assert!(receiving.received >= 5_000);
    }

    #[test]
    fn a_socket_its_program_has_closed_keeps_the_programs_name() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let client = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        let (mut server, _) = listener.accept().unwrap();
        let diag = SockDiag::default();
        let Ok(before) = diag.tcp() else {
            return eprintln!("socket diagnostics unavailable here");
        };
        let ours = |s: &&TcpCounters| s.remote.port() == port && s.remote.ip().is_loopback();
        let id = before
            .sockets
            .iter()
            .find(ours)
            .expect("the client socket")
            .id;
        // Closed here while the other end stays open: the socket lingers half-closed
        // with no file descriptor pointing at it.
        drop(client);
        let mut end = [0u8; 1];
        assert_eq!(server.read(&mut end).unwrap(), 0);
        let after = diag.tcp().unwrap();
        if let Some(lingering) = after.sockets.iter().find(|s| s.id == id) {
            assert_eq!(
                lingering.process.as_ref().map(|p| p.pid),
                Some(std::process::id()),
                "its last bytes still belong to the program that sent them"
            );
        }
    }
}
