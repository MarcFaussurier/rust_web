use std::io;
use std::io::prelude::*;
use crate::core_src::server::http_server::HttpServer;
use crate::core_src::ApplicationStates;

pub fn push_command_into_list(command: String/*, vec: &mut Vec<String>*/){

}

macro_rules! register_cmd {
($name: expr) => {
        // todo : push string into a static vector and use it inside the /help command
        // $crate::core_src::server::console::push_command_into_list(stringify!($name));??
        ($name)
    }
}


pub fn do_action (line: String, http_server: &mut HttpServer, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer {
    match line.as_str() {
        "/help" => {
            println!("You asked for help. Current commands are: /status /start /stop /pause /exit");
            return http_server;
        }

        "/status" => {
            println!("You asked for start.");
            return http_server;
        }

        "/start" => {
            println!("You asked for start.");
            return http_server.start(shared_state);
        }

        "/stop" => {
            println!("You asked for stop.");
            return http_server.stop(shared_state, String::from("User asked to stop"));
        }

        "/resume" => {
            println!("You asked for server server resuming.");
            return http_server.resume(shared_state);
        }

        "/pause" => {
            println!("You asked for pause.");
            return http_server.pause(shared_state);
        }

        "/exit" => {
            println!("You asked for exit.");
            return http_server;
        }

        _ => {
            // => Parse multiline value, handle commands with args, IE = set config key, config value
            println!("Unknow command! Type /help for command list.");
            return http_server;
        }
    }
}   

pub fn start_reading (mut http_server: HttpServer, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) {
    let stdin = io::stdin();
    loop {
        for line in stdin.lock().lines() {
            do_action(line.unwrap(), &mut http_server, shared_state.clone());
        }
    }
}