use std::sync::{Mutex, Arc};
use crate::config;
use crate::server;
use crate::worker::Worker;
use crate::action;
use crate::action_queue;

struct WorkerPoolListener {

}


impl WorkerPoolListener {

}

pub fn listen(config_instance: Arc<Mutex<config::ServerConfig>>, shared_server_state: Arc<Mutex<server::ServerState>>) {
    println!("Starting worker pool listener");


    std::thread::spawn(move || {
        let mut max_worker_count = 0;
        {
            println!("a");
            let config = config_instance.lock().unwrap();
            max_worker_count = config.thread_max_count;
            {
                println!("b");
                let ptr: Arc<Mutex<server::ServerState>> = shared_server_state.clone();
                let mut data = ptr.lock().unwrap();
                                println!("c");

                data.worker_pool.set_workers_using(max_worker_count);
                println!("c");
            }
        }
        loop {
                                println!("aaan");
                                break;

        }
      //  println!("[starting worker pool loop]");
        loop {
                    println!("aaa");
                {
                let data = shared_server_state.lock().unwrap();
                if data.worker_pool.action_queue.len() > 0 {
                    println!("non empty");

                    let mut isSet: bool = false;
                    let mut minWorker = 101;
                    let mut pos = 0;
                    for x in data.worker_pool.workers.iter() {
                        if x.action_queue.len() == 0 {
                            break;
                        } else if isSet {
                            if data.worker_pool.workers[minWorker].action_queue.len() > x.action_queue.len() {
                                minWorker = pos;
                                isSet = true;
                            }
                        } else {
                            minWorker = pos;
                        }
                    }
                    // if workerpool is busy and we can still create thread
                    if minWorker == 101 && max_worker_count > data.worker_pool.workers.len() as u64 {
                        // todo :: add temporary thread support until 
                        println!("todo :: add temporary thread support until ");
                    } 
                    
                    // else we can proceed the action
                    else {
                        println!("processing...");
                        data.worker_pool.send_action_to_worker(
                            data.worker_pool.action_queue[0], 
                            data.worker_pool.workers[minWorker].action_queue
                        )
                    } 
                } else {
                    println!("empty!");
                }
            }
           // std::thread::sleep_ms(200);
            println!("tik!");
        }
        println!("[exiting worker pool loop]");
    });
}