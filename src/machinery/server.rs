use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    loop {
        match stream.read(&mut data) {
            Ok(n) => {
                match stream.write(&data[0..n]) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("write error: {}", e);
                        break;
                    }
                };
            },
            Err(e) => {
                println!("read error: {}", e);
                break;
            }
        }
    }
}

pub fn serve(address: &str) {
    let listener = match TcpListener::bind(address) {
        Ok(x) => x,
        Err(e) => {
            println!("bind error: {}", e);
            exit(1);
        }
    };

    println!("server listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection");
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("server error: {}", e);
                exit(1);
            }
        };
    }
}
