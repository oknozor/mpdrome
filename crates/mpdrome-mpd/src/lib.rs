use mpdrome_bridge::MpdBridge;
use mpdrome_mpd_protocol::command::Command;

use crate::handler::Handler;
use crate::listener::Listener;
use std::net::TcpListener;

mod handler;
mod listener;
mod responses;

pub struct Mpd<T: MpdBridge + Clone> {
    addr: String,
    brigde: T,
}

impl<T: MpdBridge + Clone> Mpd<T> {
    pub fn new(addr: &str, bridge: T) -> Self {
        Mpd {
            addr: addr.to_string(),
            brigde: bridge,
        }
    }

    pub fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        println!("MPD server listening on {}", self.addr);

        loop {
            let (socket, addr) = listener.accept()?;
            println!("New connection from {}", addr);
            let mut listener = Listener::new(socket, self.brigde.clone());
            listener.handshake()?;

            loop {
                match listener.read_command() {
                    Ok(Some(Command { request, args })) => {
                        listener.handle(request, args)?;
                    }
                    Ok(None) => break,
                    Err(err) => {
                        eprintln!("failed to read from socket; err = {err:?}");
                    }
                }
            }
        }
    }
}
