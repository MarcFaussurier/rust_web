use crate::http_message::HttpMessage;
use std::sync::{Arc, Mutex};
use crate::config;
use crate::server;

pub struct Controller {
    pub routes: Vec<String>,
    pub callback: fn(
        message: &mut HttpMessage,
        config_instance: Arc<Mutex<config::ServerConfig>>,
        shared_server_state: Arc<Mutex<server::ServerState>>
    )-> bool,
    pub priority: i64
}
