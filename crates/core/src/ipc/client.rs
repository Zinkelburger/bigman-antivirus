//! Blocking client used by the CLI, native host and desktop shell.
use super::{
    read_frame, socket_path, write_frame, ClientFrame, Live, Request, Response, ServerFrame,
};
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
    /// A handle that cancels the request in flight on this connection: the daemon
    /// sees the client's side close, stops the job and still sends its report.
    pub fn canceller(&self) -> Result<Canceller> {
        Ok(Canceller(self.writer.try_clone()?))
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

impl Client {
    /// Subscribes on this connection and hands each push to `on` until it returns
    /// false or the daemon goes away. Use a connection of its own.
    pub fn subscribe(
        &mut self,
        events: bool,
        samples: bool,
        mut on: impl FnMut(Live) -> bool,
    ) -> Result<()> {
        let id = self.next_id;
        self.next_id += 1;
        write_frame(
            &mut self.writer,
            &ClientFrame {
                id,
                request: Request::Subscribe { events, samples },
            },
        )?;
        while let Some(frame) = read_frame::<ServerFrame>(&mut self.reader)? {
            match frame.response {
                Response::Live(live) if frame.id == id => {
                    if !on(live) {
                        return Ok(());
                    }
                }
                Response::Error(e) => bail!("{}", e.message),
                _ => return Ok(()),
            }
        }
        bail!("Bigman daemon closed the connection")
    }
}

pub struct Canceller(UnixStream);

impl Canceller {
    pub fn cancel(&self) {
        let _ = self.0.shutdown(std::net::Shutdown::Write);
    }
}

/// Where the socket lives, for messages that tell the user what was tried.
pub fn describe_socket() -> String {
    socket_path()
        .unwrap_or_else(|_| PathBuf::from("$XDG_RUNTIME_DIR/bigman/bigmand.sock"))
        .display()
        .to_string()
}
