mod app_src;
mod core_src;

use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use core_src::ApplicationStates;
use core_src::server::console::start_reading;


fn main() {
    let http_server = app_src::server::services::http_server::HttpServer {
        ip: String::from("0.0.0.0"),
        port: 8090
    };
    
    let inital_state    = ApplicationStates { should_exit: false, exit_message: String::from("") };
    let shared_state    = Arc::new(Mutex::new(inital_state));

    println!("Welcome in WebRust, type /help for command list.");
    start_reading(http_server, shared_state.clone());
}
