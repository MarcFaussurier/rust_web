use crate::action::DeferedAction;
use std::sync::{Arc, Mutex};

pub type ActionQueue = Vec<Arc<Mutex<DeferedAction>>>;
