use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::core_src::ApplicationStates;
use std::time::{Duration, Instant};
extern crate chrono;
use chrono::*;

pub struct HttpServer {
    pub ip: String,
    pub port: u16,
}

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

fn handle_client(stream: TcpStream)  {
    handle_read(&stream);
    handle_write(stream);
}

impl HttpServer {
    pub fn start(&mut self, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer  {
        {
            let mut data = shared_state.lock().unwrap();
            data.should_exit = false;
            data.is_paused = false;
        }
    
        println!("starting server...");
        thread::spawn(move || {
            let listener = &mut TcpListener::bind("127.0.0.1:8090");
            match listener {
                Ok(e) => {
                    println!("Success for listening");
                    let mut is_paused = false;
                    loop {
                        let data = shared_state.lock().unwrap();
                        if !data.is_paused {
                            if is_paused {
                                println!("Server resumed");
                            }

                            for stream in e.incoming() {
                                match stream {
                                    Ok(stream) => {
                                        handle_client(stream);
                                    }
                                    Err(e) => { println!("error {:?}", e); }
                                }
                            }

                            if  data.should_exit {
                                println!("server exited");
                                break;
                            }
                            is_paused = false;
                        } else {
                            if !is_paused {
                                println!("Server paused");
                            }
                            is_paused = true;
                        }
                    }
                }

                Err(e) => {
                    println!("Failed for listening: {}", e);
                    return;
                }
            }
        });

        return self;
    }

    pub fn pause(&mut self, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer {
        let mut data = shared_state.lock().unwrap();
        data.is_paused = true;   
        return self;
    }

    pub fn resume(&mut self, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer {
        let mut data = shared_state.lock().unwrap();
        data.is_paused = false;   
        return self;
    }

    pub fn stop(&mut self, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>, exit_reason: String) -> &mut HttpServer {
        let mut data = shared_state.lock().unwrap();
        data.should_exit = true;   
        data.exit_message = format!("Exiting ... [{}]", exit_reason);
        return self;
    }

    pub fn status(&mut self, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer {
        let mut data = shared_state.lock().unwrap();
        if data.should_exit {
            println!("Server is stoped.");
        } else if data.is_paused {
            println!("Server is paused.");
        } else {
            match TcpStream::connect(format!("{}:{}", self.ip, self.port)) {
                Ok(mut stream) => {
                    println!("Successfully connected to server in port {}", self.port);
                    stream.write(b"Hello!").unwrap();
                    let start = Instant::now();
                    println!("Sent Hello, awaiting reply...");

                    let mut data = [0 as u8; 6]; // using 6 byte buffer
                    match stream.read_exact(&mut data) {
                        Ok(_) => {
                            let duration = start.elapsed();
                            println!("PING: {:?}", duration);
                        },
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
        }
        

        return self;
    }
}