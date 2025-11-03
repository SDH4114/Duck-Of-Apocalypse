pub mod local_pty;
pub mod ssh_backend;

pub trait SessionBackend: Send {
    fn poll_output(&mut self) -> Option<String>;
    fn send_input(&mut self, data: &str);
    fn is_alive(&self) -> bool;
    fn close(&mut self);
}