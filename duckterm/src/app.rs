use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph, Tabs},
    text::Line,
    style::{Style, Color},
};

use crate::session::Session;
use crate::backends::local_pty::LocalPtyBackend;
use crate::backends::ssh_backend::SshBackend;
use crate::commands::{parse_command, CommandAction};

pub fn run() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sessions: Vec<Session> = vec![Session::new("local", Box::new(LocalPtyBackend::new("zsh")?))];
    let mut active_idx: usize = 0;
    let mut cmd_buf = String::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(120);

    'main: loop {
        for s in sessions.iter_mut() {
            s.tick();
        }

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)])
                .split(size);

            let titles: Vec<Line> = sessions.iter().enumerate().map(|(i, s)| {
                if i == active_idx {
                    Line::from(format!(" {} ", s.title)).style(Style::default().fg(Color::Yellow))
                } else {
                    Line::from(format!(" {} ", s.title))
                }
            }).collect();

            f.render_widget(Tabs::new(titles).block(Block::default().borders(Borders::ALL).title("sessions")), chunks[0]);
            f.render_widget(
                Paragraph::new(sessions[active_idx].buffer.join("")).block(
                    Block::default().borders(Borders::ALL).title(sessions[active_idx].title.clone())
                ),
                chunks[1],
            );
            f.render_widget(
                Paragraph::new(format!(":{}", cmd_buf)).block(Block::default().borders(Borders::ALL).title("cmd (':' = internal)")),
                chunks[2],
            );
        })?;

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(Duration::from_millis(0));
        if event::poll(timeout)? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char(c) => cmd_buf.push(c),
                        KeyCode::Backspace => { cmd_buf.pop(); },
                        KeyCode::Enter => {
                            let line = cmd_buf.trim().to_string();
                            cmd_buf.clear();

                            if line.starts_with(':') {
                                let (msg, action) = parse_command(&line[1..]);
                                if !msg.is_empty() {
                                    sessions[active_idx].buffer.push(msg + "\n");
                                }

                                match action {
                                    CommandAction::None => {}
                                    CommandAction::NewLocal(shell) => match LocalPtyBackend::new(&shell) {
                                        Ok(b) => {
                                            sessions.push(Session::new(&shell, Box::new(b)));
                                            active_idx = sessions.len() - 1;
                                        }
                                        Err(e) => sessions[active_idx].buffer.push(format!("err: {e}\n")),
                                    },
                                    CommandAction::NewSSH(host) => match SshBackend::new(&host) {
                                        Ok(b) => {
                                            sessions.push(Session::new(&format!("ssh:{host}"), Box::new(b)));
                                            active_idx = sessions.len() - 1;
                                        }
                                        Err(e) => sessions[active_idx].buffer.push(format!("ssh err: {e}\n")),
                                    },
                                    CommandAction::Quit => break 'main,
                                }
                            } else {
                                sessions[active_idx].buffer.push(format!("$ {}\n", line));
                                sessions[active_idx].send(&(line + "\n"));
                            }
                        }
                        KeyCode::Esc => cmd_buf.clear(),
                        KeyCode::Char('q') if cmd_buf.is_empty() => break 'main,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}