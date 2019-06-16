use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use crate::core_src::ApplicationStates;
/*
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
}s
*/

pub struct HttpServer {
    pub ip: String,
    pub port: u16,
}

impl HttpServer {
    pub fn start(&mut self, dataRef: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) {
        let innerThread = thread::spawn(move || {
            loop {
                let mut data = dataRef.lock().unwrap();
                println!("CurrentValue = {}", data.shouldExit);
                if  data.shouldExit {
                    break
                }
                thread::sleep(Duration::from_millis(5));
            }
        });
    }

    pub fn stop(&mut self, dataRef: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) {
        let mut data = dataRef.lock().unwrap();
        data.shouldExit = true;   
    }
}