use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let ip_address = match env::var("SERVER_IP") {
        Ok(var) => var,
        Err(e) => panic!("{}", e)
    };

    let listener = TcpListener::bind(ip_address)
        .expect("Not possible to connect to selected ip");
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(&mut stream);
    }
}

fn handle_request(stream: &mut TcpStream) {
    let mut buffer = [0 as u8; 1024]; //1KB

    let buffer_size = match stream.read(&mut buffer) {
        Ok(stream_length) => stream_length,
        Err(e) => panic!("{}", e)
    };

    println!("The request is: {}", String::from_utf8_lossy(&buffer[0..buffer_size]));

    let response = format!(
        "HTTPS/1.1 200 OK\r\n\r\n",
    );

    match stream.write(response.as_bytes()) {
        Ok(size) => println!("Written to HTTP stream successfully, {} bytes were written", size),
        Err(e) => panic!("{}", e)
    }
}