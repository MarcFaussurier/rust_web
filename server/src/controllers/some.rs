extern crate lib;

use lib::controller_stack::ControllerStack;
use lib::controller::Controller;
use lib::http_message::HttpMessage;
use std::sync::{Arc, Mutex};
use crate::config;
use crate::server;

pub fn register(stack: &mut ControllerStack) {


    stack.controllers.push(Controller {
        routes: vec![ String::from("*"), String::from("*") ],
        callback:
        |   message: &mut HttpMessage,
            config_instance: Arc<Mutex<config::ServerConfig>>,
            shared_server_state: Arc<Mutex<server::ServerState>>
        |-> bool {
            message.body = String::from("Hello world");
            return false;
            // if true, we release the response,
            // else we keep update it by decreasing the priority
        },
        priority: 99
    });

    stack.controllers.push(Controller {
        routes: vec![ String::from("/hello"), String::from("/world") ],
        callback:
        |   message: &mut HttpMessage,
            config_instance: Arc<Mutex<config::ServerConfig>>,
            shared_server_state: Arc<Mutex<server::ServerState>>
        |-> bool {
            message.body = String::from("Hello world with ++ priority");
            return false;
        },
        priority: -99
    });

}
