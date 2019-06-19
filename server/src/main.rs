extern crate lib;

use std::thread;
use std::sync::{Mutex, Arc};
use lib::config;
use lib::server;
use lib::{https_listener, http_listener, wss_listener, ws_listener, console_reader, worker_pool, worker_pool_listener};

static NTHREADS: i32 = 10;

fn main() {
   // server datas
   let config_instance: config::ServerConfig = config::read(String::from("dev"));
   let mut server_state = server::ServerState {
      stopped: false,
      paused: false,
      reason:  "Server is not started yet",
      worker_pool: worker_pool::WorkerPool {
         workers: vec![],
         action_queue: vec![]
      }
   };
   let mut shared_server_state = Arc::new(Mutex::new(server_state));
   let mut shared_config = Arc::new(Mutex::new(config_instance));
    // Make a vector of thread for all main threads (a thread pool is a main thread that uses child threads)
   let mut main_threads = vec![];

   {
      let a = shared_config.clone();
      let b = shared_server_state.clone();
      main_threads.push(thread::spawn(move || {
         worker_pool_listener::listen(a, b);
      }));
   }{
      let a = shared_config.clone();
      let b = shared_server_state.clone();
      main_threads.push(thread::spawn(move || {
         http_listener::listen(a, b);
      }));
   }{
      let a = shared_config.clone();
      let b = shared_server_state.clone();
   
      main_threads.push(thread::spawn(move || {
         https_listener::listen(a, b);
      }));
   }{
      let a = shared_config.clone();
      let b = shared_server_state.clone();
      main_threads.push(thread::spawn(move || {
         ws_listener::listen(a, b);
      }));
   }{
      let a = shared_config.clone();
      let b = shared_server_state.clone();
      main_threads.push(thread::spawn(move || {
         wss_listener::listen(a, b);
      }));
   }{
      let a = shared_config.clone();
      let b = shared_server_state.clone();
      main_threads.push(thread::spawn(move || {
         console_reader::listen(a, b);
      }));
   }


   loop {
      {
      let mut data = shared_server_state.lock().unwrap();
      if data.stopped {
         break;
      }
   
      std::thread::sleep_ms(200);
      println!("sending action....");

      data.worker_pool.action_queue.push(|| {
         println!("action");
      });

      println!("sent....");
      }      
      std::thread::sleep_ms(200);



   }


    for child in main_threads {
        // Wait for the thread to finish. Returns a result.
         let _ = child.join();
    }

    println!("Exiting...");
}
