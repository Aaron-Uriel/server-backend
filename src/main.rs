#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use dotenv::dotenv;
use std::env;

mod consts;
use consts::*;

mod mylib;

fn main() {
    let connection = mylib::create_connection();

    dotenv().ok();

    let ip_address = match env::var("SERVER_IP") {
        Ok(var) => var,
        Err(e) => panic!("{}", e)
    };

    let listener = TcpListener::bind(ip_address)
        .expect("Not possible to connect to selected ip");
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(&mut stream, &connection);
    }
}

fn handle_request(stream: &mut TcpStream, conn: &diesel::MysqlConnection) {

    let mut buffer = [0 as u8; 1024]; //1KB

    let buffer_size = match stream.read(&mut buffer) {
        Ok(stream_length) => stream_length,
        Err(e) => panic!("{}", e)
    };

    println!("The request is: {}", String::from_utf8_lossy(&buffer[0..buffer_size]));

    let (return_code, json_response) = if buffer.starts_with(resquests::Food.as_bytes()) {
        let food_list = mylib::get_food_vec(conn);
        (returns::Ok, serde_json::to_string_pretty(&food_list).unwrap())
    } 
    else if buffer.starts_with(resquests::Variants.as_bytes()) {
        let variants_list = mylib::get_variants_vec(conn);
        (returns::Ok, serde_json::to_string_pretty(&variants_list).unwrap())
    }
    else if buffer.starts_with(resquests::Tables.as_bytes()) {
        let tables_list = mylib::get_tables_vec(conn);
        (returns::Ok, serde_json::to_string_pretty(&tables_list).unwrap())
    }
    else {
        (returns::NotFound, String::from(""))
    };

    //Es el texto HTTP que se va a devolver
    //HTTP exige que las lineas terminen con \r\n el \ adicional es
    //para evitar que las tabulaciones a la izquierda también aparezcan xd
    let response = format!(
        "{}\r\n\
         Content-type: application/json\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        return_code,
        json_response.len(),
        json_response
    );

    println!("The response will be:\n{}\nEnd of the response", response);

    match stream.write(response.as_bytes()) {
        Ok(size) => println!("\nWritten to HTTP stream successfully, {} bytes were written", size),
        Err(e) => panic!("{}", e)
    }
}