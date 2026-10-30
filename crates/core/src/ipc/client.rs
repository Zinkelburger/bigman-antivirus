//! Blocking client used by the CLI, native host and desktop shell.
use super::{read_frame, socket_path, write_frame, ClientFrame, Request, Response, ServerFrame};
use anyhow::{bail, Context, Result};
use std::{
    io::BufReader,
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    time::Duration,
};

pub struct Client {
    reader: BufReader<UnixStream>,
    writer: UnixStream,
    next_id: u64,
}

impl Client {
    pub fn connect() -> Result<Self> {
        Self::connect_to(&socket_path()?)
    }
    pub fn connect_to(path: &Path) -> Result<Self> {
        let stream = UnixStream::connect(path)
            .with_context(|| format!("Bigman daemon is not running at {}", path.display()))?;
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            writer: stream,
            next_id: 1,
        })
    }
    /// Bounds how long a single request may wait for its response.
    pub fn set_timeout(&self, timeout: Option<Duration>) -> Result<()> {
        self.writer.set_read_timeout(timeout)?;
        Ok(())
    }
    pub fn request(&mut self, request: Request) -> Result<Response> {
        let id = self.next_id;
        self.next_id += 1;
        write_frame(&mut self.writer, &ClientFrame { id, request })?;
        let Some(frame): Option<ServerFrame> = read_frame(&mut self.reader)? else {
            bail!("Bigman daemon closed the connection");
        };
        if frame.id != id {
            bail!(
                "Daemon answered request {} while {id} was pending",
                frame.id
            );
        }
        Ok(frame.response)
    }
}

/// Where the socket lives, for messages that tell the user what was tried.
pub fn describe_socket() -> String {
    socket_path()
        .unwrap_or_else(|_| PathBuf::from("$XDG_RUNTIME_DIR/bigman/bigmand.sock"))
        .display()
        .to_string()
}
