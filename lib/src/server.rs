use crate::worker_pool::WorkerPool;

pub struct ServerState {
   pub stopped: bool,
   pub paused: bool,
   pub reason: &'static str,
   pub worker_pool: WorkerPool,
}