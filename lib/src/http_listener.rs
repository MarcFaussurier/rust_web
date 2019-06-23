use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::http_message::{read, write, parse_first_line};
use crate::action::ConcurrentAction;
use std::rc::Rc;
use std::cell::RefCell;

struct HttpListener {

}

fn handle_client(stream: &mut TcpStream) {
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

    pub struct test {
        pub cb: TcpStream
    }

    for mut stream in listener.incoming() {
        let mut stream_arc = Arc::new(Mutex::new(test{ cb: stream.unwrap()}));

        let action = Arc::new(Mutex::new(ConcurrentAction::new(
            move || {
                println!("PATATE4");
                let mut stream = stream_arc.lock().unwrap();
                let mut inner = &mut stream.cb;
                handle_client(&mut inner);
        })));
        let mut data = shared_server_state.lock().unwrap();
        data.worker_pool.action_queue.push(action);
    }

}
