use std::{
    io::{Read, Write},
    net::TcpStream,
    process::exit,
};

pub fn connect(address: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let data = "hello server!".as_bytes();
            match stream.write(data) {
                Ok(_) => {
                    let mut data = [0 as u8; 1024];
                    match stream.read(&mut data) {
                        Ok(n) => {
                            println!("{}", String::from_utf8_lossy(&data[0..n]));
                        },
                        Err(e) => {
                            println!("read error: {}", e);
                        }
                    };
                },
                Err(e) => {
                    println!("write error: {}", e);
                    return;
                }
            }
        },
        Err(e) => {
            println!("connection error: {}", e);
            exit(1);
        }
    };
}
