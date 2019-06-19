use crate::worker;
use crate::config;
use crate::action;
use crate::action_queue;
use crate::worker::Worker;

use std::sync::{Mutex, Arc};

pub struct WorkerPool {
    pub workers:        Vec<worker::Worker>,
    pub action_queue:   action_queue::ActionQueue
}

impl WorkerPool {
    pub fn set_workers_using(&mut self, default_mac_cpu: u64) {
        for x in 0 .. default_mac_cpu {
            self.workers.push(Worker{action_queue: vec![]});
        }
    }

    pub fn send_action_to_worker(&self, action: action::DeferedAction, queue: &mut action_queue::ActionQueue) {
        queue.push(action.callback);
    }
}

