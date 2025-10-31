use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use mpdrome_bridge::{BridgeError, MpdBridge};

use crate::protocol::requests::{Command, Request, error::CommandError};

pub struct Listener<T: MpdBridge + Clone> {
    inner: TcpStream,
    buffer: [u8; 1024],
    bridge: T,
}

impl<T: MpdBridge + Clone> Read for Listener<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<T: MpdBridge + Clone> Write for Listener<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T: MpdBridge + Clone> Listener<T> {
    pub fn new(inner: TcpStream, bridge: T) -> Self {
        Listener {
            inner,
            bridge,
            buffer: [0u8; 1024],
        }
    }

    pub fn bridge(&self) -> &T {
        &self.bridge
    }

    pub fn handshake(&mut self) -> io::Result<()> {
        self.inner.write_all(b"OK MPD 0.26.0\n")?;
        Ok(())
    }

    pub fn read_command(&mut self) -> ListenerResult<Option<Command>> {
        let n = self.inner.read(&mut self.buffer)?;
        if n == 0 {
            return Ok(None);
        }

        let request = String::from_utf8_lossy(&self.buffer[..n]);

        Command::parse(&request)
            .map(Option::Some)
            .map_err(ListenerError::from)
    }

    pub fn idle_wait(&mut self) -> ListenerResult<()> {
        loop {
            let command = self.read_command()?;
            if let Some(Command {
                request: Request::NoIdle,
                ..
            }) = command
            {
                break;
            }
        }
        Ok(())
    }
}

pub type ListenerResult<T> = Result<T, ListenerError>;

#[derive(Debug, thiserror::Error)]
pub enum ListenerError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Command error: {0}")]
    Command(#[from] CommandError),

    #[error("Bridge error: {0}")]
    Bridge(#[from] BridgeError),
}
