use std::{
    io::{Read, Write},
    net::TcpStream
};
use anyhow::{Result, Context};

pub fn connect(address: &str) -> Result<()> {
    let mut stream = TcpStream::connect(address).with_context(||format!("cannot connect to {}", address))?;
    let data = "hello server!".as_bytes();
    stream.write(data).context("cannot write data to server")?;
    let mut data = [0 as u8; 1024];
    let n = stream.read(&mut data).context("cannot read data from server")?;
    println!("{}", String::from_utf8_lossy(&data[0..n]));

    Ok(())
}
