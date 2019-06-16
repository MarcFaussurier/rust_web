mod app_src;
mod core_src;

use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use core_src::ApplicationStates;

static enableStop: bool = true;

fn main() {
    let mut srv = app_src::server::services::http_server::HttpServer {
        ip: String::from("0.0.0.0"),
        port: 8090
    };
    
    let initialState = ApplicationStates { shouldExit: false, exitMessage: String::from("") };
    let data = Arc::new(Mutex::new(initialState));
    let dataRef = data.clone();

    srv.start(dataRef);
    thread::sleep(Duration::from_millis(50));
    let dataRef = data.clone();

    if enableStop {
        srv.stop(dataRef);
    }

    thread::sleep(Duration::from_millis(50));

    println!("Hello, world!");
}
