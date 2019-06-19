use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;

struct WssListener {

}


pub fn listen(config_instance: Arc<Mutex<config::ServerConfig>>, shared_server_state: Arc<Mutex<server::ServerState>>) {

}