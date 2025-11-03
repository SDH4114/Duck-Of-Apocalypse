use crate::backends::SessionBackend;

pub struct Session {
    pub title: String,
    pub buffer: Vec<String>,
    backend: Box<dyn SessionBackend>,
}

impl Session {
    pub fn new(title: &str, backend: Box<dyn SessionBackend>) -> Self {
        Self {
            title: title.to_string(),
            buffer: Vec::new(),
            backend,
        }
    }

    pub fn tick(&mut self) {
        if let Some(out) = self.backend.poll_output() {
            self.buffer.push(out);
        }
    }

    pub fn send(&mut self, data: &str) {
        self.backend.send_input(data);
    }
}