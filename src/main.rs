use anyhow::{Context, Result};
use std::thread::sleep;
use std::time::Duration;

fn get_ip() -> Result<String> {
    std::env::args().skip(1).next().context("Requires IP")
}

#[cfg(feature = "server")]
fn main() -> Result<()> {
    let client_ip = get_ip()?;
    let board = wiiboard::WiiBoard::new(5)?;
    let socket = std::net::UdpSocket::bind("127.0.0.1:4096")?;
    loop {
        if let Some(board_msg) = board.poll()? {
            let msg = bincode::serialize(&wiiuse_remote_hack::Message {
                top_left: board_msg.top_left,
                top_right: board_msg.top_right,
                bottom_left: board_msg.bottom_left,
                bottom_right: board_msg.bottom_right,
            })?;
            socket.send_to(&msg, &client_ip)?;
            sleep(Duration::from_millis(10));
        } else {
            sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(not(feature = "server"))]
fn main() -> Result<()> {
    let mut client = wiiuse_remote_hack::WiiBoardClient::new(get_ip()?)?;
    loop {
        if let Some(data) = client.poll()? {
            dbg!(data);
        } else {
            sleep(Duration::from_millis(10));
        }
    }
}
