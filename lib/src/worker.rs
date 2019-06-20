use crate::action_queue;
use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;

pub struct Worker {
    pub action_queue:   action_queue::ActionQueue
}