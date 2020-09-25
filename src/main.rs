use std::net::UdpSocket;
use anyhow::Result;
use serde::{Deserialize, Serialize};

fn main() -> Result<()> {
    if std::env::args().skip(1).next().is_some() {
        server()
    } else {
        client()
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Message {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

fn server() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:12345")?;
    let mut buf = [0; 256];
    loop {
        let (n_bytes, _addr) = socket.recv_from(&mut buf)?;
        let msg = &buf[..n_bytes];
        if let Ok(data) = bincode::deserialize::<Message>(&msg) {
            dbg!(n_bytes);
            dbg!(data);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn client() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:1234")?;
    let mut a = 0;
    loop {
        let msg = bincode::serialize(&Message { a, b: a+1, c: a+2, d:a*2 })?;
        socket.send_to(&msg, "127.0.0.1:12345")?;
        std::thread::sleep(std::time::Duration::from_millis(1));
        a += 1;
    }
}
