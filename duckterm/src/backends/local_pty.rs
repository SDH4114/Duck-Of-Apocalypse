use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use portable_pty::{CommandBuilder, native_pty_system};
use crate::backends::SessionBackend;

pub struct LocalPtyBackend {
    reader_buf: Arc<Mutex<Vec<String>>>,
    writer: Box<dyn Write + Send>,
    alive: Arc<Mutex<bool>>,
}

impl LocalPtyBackend {
    pub fn new(shell_cmd: &str) -> anyhow::Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(Default::default())?;

        let mut cmd = CommandBuilder::new(shell_cmd);
        let mut child = pair.slave.spawn_command(cmd)?;

        let reader_buf = Arc::new(Mutex::new(Vec::<String>::new()));
        let alive = Arc::new(Mutex::new(true));

        let mut reader = pair.master.try_clone_reader()?;
        let reader_buf_cl = Arc::clone(&reader_buf);
        let alive_cl = Arc::clone(&alive);
        thread::spawn(move || {
            let mut tmp = [0u8; 4096];
            loop {
                match reader.read(&mut tmp) {
                    Ok(0) => {
                        *alive_cl.lock().unwrap() = false;
                        break;
                    }
                    Ok(n) => {
                        let s = String::from_utf8_lossy(&tmp[..n]).to_string();
                        reader_buf_cl.lock().unwrap().push(s);
                    }
                    Err(_) => {
                        *alive_cl.lock().unwrap() = false;
                        break;
                    }
                }
            }
        });

        let writer = pair.master.take_writer()?;
        {
            let alive_cl = Arc::clone(&alive);
            thread::spawn(move || {
                let _ = child.wait();
                *alive_cl.lock().unwrap() = false;
            });
        }

        Ok(Self { reader_buf, writer, alive })
    }
}

impl SessionBackend for LocalPtyBackend {
    fn poll_output(&mut self) -> Option<String> {
        let mut data = self.reader_buf.lock().unwrap();
        if !data.is_empty() {
            Some(data.drain(..).collect::<Vec<_>>().join(""))
        } else {
            None
        }
    }

    fn send_input(&mut self, data: &str) {
        let _ = self.writer.write_all(data.as_bytes());
        let _ = self.writer.flush();
    }

    fn is_alive(&self) -> bool {
        *self.alive.lock().unwrap()
    }

    fn close(&mut self) {
        *self.alive.lock().unwrap() = false;
    }
}