use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};
use anyhow::{Context, Result};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut data = [0 as u8; 1024];
    loop {
        let n = stream.read(&mut data).context("cannot read data from client")?;
        stream.write(&data[0..n]).context("cannot write data to client")?;
    }
}

pub fn serve(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address).with_context(||format!("cannot use address {}", address))?;
    println!("server listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let stream = stream.context("cannot handle incoming connection")?;
        println!("new connection");
        thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|e|println!("{:?}", e));
        });
    }

    Ok(())
}
