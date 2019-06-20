use crate::worker;
use crate::config;
use crate::action::DeferedAction;
use crate::action_queue;
use crate::worker::{Worker};
use std::thread;
use std::sync::{Mutex, Arc};
use crate::server;
pub struct WorkerPool {
    pub workers:        Vec<Arc<Mutex<worker::Worker>>>,
    pub action_queue:   action_queue::ActionQueue
}

impl WorkerPool {
    pub fn set_workers_using(&mut self, config_instance: Arc<Mutex<config::ServerConfig>>, shared_server_state: Arc<Mutex<server::ServerState>>) {
        let mut thread_count = 0;
        {
            let data = config_instance.lock().unwrap();
            thread_count = data.thread_count;
        }
        for x in 0 .. thread_count {
            let mut worker = Worker{action_queue: vec![]};
            let mut data = Arc::new(Mutex::new(worker));
            self.workers.push(data.clone());
            let mut worker = data.clone();

            std::thread::spawn(move || {
                loop {
                    let mut data = worker.lock().unwrap();

                    // if there are some action to proceed 
                    if data.action_queue.len() > 0 {
                        // we retrive and proceed the first one
                        {
                            let mut action = data.action_queue[0].lock().unwrap();
                            (action.callback)();
                        }
                        // then we remove it from the stack
                        data.action_queue.remove(0);
                    } 
                    // there is no action in the queue
                    else {

                    }   
                }
            });
        } 
    }
}

