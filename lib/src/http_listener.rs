use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::http_message::{read, write, parse_first_line};
struct HttpListener {

}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0 ;4096];
    stream.read(&mut buf).unwrap();
    let s = match String::from_utf8(buf.to_vec()) {
        Ok(v) => {
            println!("{} ", v);
            let output = read(v);

            println!("url: {}", parse_first_line(&output).uri);
            println!("type: {}", parse_first_line(&output).req_type);
            println!("protocol: {}", parse_first_line(&output).protocol);

            println!("{}", write(output));
        },
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e); 
        },
    };
}

pub fn listen(config_instance: Arc<Mutex<config::ServerConfig>>, shared_server_state: Arc<Mutex<server::ServerState>>) {
  
    let mut ip: String = String::from("0.0.0.0");
    let mut port: u16 = 8080;

    {
        let  mut data = config_instance.lock().unwrap();
        ip = data.http_ip.clone();
        port = data.http_port as u16;
    }


    println!("Listening http on {}:{}", ip, port);
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();

    for stream in listener.incoming() {
        // It's a single connection server :)
        // Ok, we could spawn a thread here but
        // stdout is blocked anyways (see handle_client).
        handle_client(stream.expect("failed to unwrap stream"));
    }
}