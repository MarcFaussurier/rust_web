pub mod server;
pub mod component_renderer;

pub struct ApplicationStates {
    pub is_paused: bool,
    pub should_exit: bool,
    pub exit_message: String
}