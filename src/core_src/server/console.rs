use std::io;
use std::io::prelude::*;
use crate::core_src::server::http_server::HttpServer;
use crate::core_src::ApplicationStates;
use crate::core_src::component_renderer::parser::parse_source_code;

pub fn do_action (line: String, http_server: &mut HttpServer, shared_state: std::sync::Arc<std::sync::Mutex<ApplicationStates>>) -> &mut HttpServer {
    match line.as_str() {
        "/help" => {
            println!("You asked for help. Current commands are: /status /start /stop /pause /exit /parse_comoponents");
            return http_server;
        }

        "/parse_components" => {
            parse_source_code(&mut String::from("MySuperComponent.rt"));
            return http_server;
        }

        "/status" => {
            println!("You asked for status.");
            return http_server.status(shared_state);
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
            let http_server = http_server.stop(shared_state, String::from("User asked to stop"));
            ::std::process::exit(0);
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