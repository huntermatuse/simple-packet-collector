use chrono::Utc;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::UdpSocket;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:20777")?;
    let mut buf = [0u8; 2048];

    let filename = format!("packets_{}.bin", Utc::now().format("%Y-%m-%d_%H-%M-%S"));
    let file = OpenOptions::new().create(true).append(true).open(&filename)?;
    let mut writer = BufWriter::new(file);

    let mut last_time = Instant::now();

    loop {
        let (amt, _) = socket.recv_from(&mut buf)?;

        let now = Instant::now();
        let delta_us = now.duration_since(last_time).as_micros() as u64;
        last_time = now;

        // [delta (8 bytes)] [len (2 bytes)] [data]
        writer.write_all(&delta_us.to_le_bytes())?;
        let len = amt as u16;
        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&buf[..amt])?;
    }
}
