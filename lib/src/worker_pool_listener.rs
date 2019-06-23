use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;
use crate::worker::Worker;
use crate::action;
use crate::action_queue;
use crate::action::ConcurrentAction;

struct WorkerPoolListener {

}


impl WorkerPoolListener {

}

pub fn listen(config_instance: Arc<Mutex<config::ServerConfig>>, shared_server_state: Arc<Mutex<server::ServerState>>) {
    println!("Starting worker pool listener");

    let mut remove_first: bool = false;
    std::thread::spawn(move || {
        let mut max_worker_count = 0;
        {let ptr: Arc<Mutex<server::ServerState>> = shared_server_state.clone();
            let mut data = ptr.lock().unwrap();
            data.worker_pool.set_workers_using(config_instance.clone(), shared_server_state.clone());
        }
        loop {
                {let mut data = shared_server_state.lock().unwrap();
                if data.worker_pool.action_queue.len() > 0 {

                    let mut minWorker = 0;
                    let mut y = 0;
                    println!("trying to find min worker...");
                    for x in data.worker_pool.workers.iter() {
                        {
                            {let mut current_worker = x.lock().unwrap();
                                {
                                    if  minWorker != y {
                                        {let mut worker =    data.worker_pool.workers[minWorker].lock().unwrap();
                                            if worker.action_queue.len() > current_worker.action_queue.len() {
                                                minWorker = y;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        y = y + 1;
                    }
                    let mut min_worker =    data.worker_pool.workers[minWorker].lock().unwrap();
                    println!("min worker: {}", minWorker);
                    // if target worker is busy     and     if we can still create new thread
                    if min_worker.action_queue.len() > 0 && max_worker_count > data.worker_pool.workers.len() as u64 {
                        // todo :: add temporary thread support until
                        println!("todo :: add temporary thread support until ");
                    }
                    // else we can proceed the action
                    else {
                        println!("processing...");
                        // push the action to the raget worker queue
                        let mut v: Arc<Mutex<ConcurrentAction>> = data.worker_pool.action_queue[0].clone();
                        min_worker.action_queue.push(v);
                        // remove current action
                        remove_first = true;
                    }
                }
                // thread pool action  queue is now empty
                else {
                 //   println!("empty!");
                }
            }

            if remove_first {
                remove_first = false;
                {
                    let mut c = shared_server_state.clone();
                    let mut data = c.lock().unwrap();
                    data.worker_pool.action_queue.remove(0);
                }
            }

        }
        println!("[exiting worker pool loop]");
    });
}
