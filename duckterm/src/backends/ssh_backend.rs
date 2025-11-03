use std::io::{Read, Write};
use std::net::TcpStream;
use ssh2::Session as SshSession;
use crate::backends::SessionBackend;

pub struct SshBackend {
    session: SshSession,
    channel: ssh2::Channel,
}

impl SshBackend {
    pub fn new(conn_str: &str) -> anyhow::Result<Self> {
        let tcp = TcpStream::connect(conn_str)?;
        let mut session = SshSession::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        // TODO: login (user@host)
        session.userauth_agent("root")?;

        let mut channel = session.channel_session()?;
        channel.request_pty("xterm", None, None)?;
        channel.shell()?;

        Ok(Self { session, channel })
    }
}

impl SessionBackend for SshBackend {
    fn poll_output(&mut self) -> Option<String> {
        let mut buf = String::new();
        match self.channel.read_to_string(&mut buf) {
            Ok(_) if !buf.is_empty() => Some(buf),
            _ => None,
        }
    }

    fn send_input(&mut self, data: &str) {
        let _ = self.channel.write_all(data.as_bytes());
        let _ = self.channel.flush();
    }

    fn is_alive(&self) -> bool {
        !self.channel.eof()
    }

    fn close(&mut self) {
        let _ = self.channel.close();
    }
}