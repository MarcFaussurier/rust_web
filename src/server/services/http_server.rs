use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn listen() -> Result<i16, std::io::Error> {
    
    let listener = TcpListener::bind("127.0.0.1:8090")?;

    let shouldExit = false;

    println!("Listening...");

    while ! shouldExit {
        for stream in listener.incoming() {
            println!("Got connexion");
            handle_client(stream?);
        }
        if (shouldExit) {
            break;
        }
    }

    return Ok(5);
}

pub struct HttpServer {
    pub ip: String
}

impl HttpServer {
    pub fn start(self) {
       match listen() {
            Ok(t)  => print!("Success"),
            Err(e) => print!("Err {}", e),
        }
    }

    pub fn stop(self) {

    }
}