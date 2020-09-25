use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::{UdpSocket, ToSocketAddrs};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Message {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

pub struct WiiBoardClient {
    socket: UdpSocket,
    buffer: [u8; 16],
}

impl WiiBoardClient {
    pub fn new(ip: impl ToSocketAddrs) -> Result<Self> {
        let socket = UdpSocket::bind(ip)?;
        socket.set_nonblocking(true)?;
        Ok(Self {
            socket,
            buffer: [0; 16],
        })
    }

    pub fn poll(&mut self) -> Result<Option<Message>> {
        let n_bytes = recv_latest(&self.socket, &mut self.buffer)?;
        if n_bytes < self.buffer.len() {
            return Ok(None);
        }
        Ok(bincode::deserialize::<Message>(&self.buffer).map(Some)?)
    }
}

fn recv_latest(socket: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
    let mut n_bytes = 0;
    loop {
        match socket.recv_from(buf) {
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    break Ok(n_bytes);
                } else {
                    Err(e)?;
                }
            }
            Ok((n, _)) => n_bytes = n,
        }
    }
}
