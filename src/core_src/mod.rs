pub mod server;

pub struct ApplicationStates {
    pub is_paused: bool,
    pub should_exit: bool,
    pub exit_message: String
}