use crate::action::ConcurrentAction;
use std::sync::{Arc, Mutex};

pub type ActionQueue = Vec<Arc<Mutex<ConcurrentAction>>>;
