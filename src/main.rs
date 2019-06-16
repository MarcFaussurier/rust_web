mod app_src;
mod core_src;


fn main() {
    let http_server = core_src::server::http_server::HttpServer {
        ip: String::from("0.0.0.0"),
        port: 8090
    };
    
    let inital_state    = core_src::ApplicationStates { is_paused: false, should_exit: false, exit_message: String::from("") };
    let shared_state    = std::sync::Arc::new(std::sync::Mutex::new(inital_state));

    println!("Welcome in WebRust, type /help for command list.");
    core_src::server::console::start_reading(http_server, shared_state.clone());
}
