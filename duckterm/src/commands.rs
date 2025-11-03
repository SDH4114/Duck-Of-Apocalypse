pub enum CommandAction {
    None,
    NewLocal(String),
    NewSSH(String),
    Quit,
}

pub fn parse_command(line: &str) -> (String, CommandAction) {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return ("".into(), CommandAction::None);
    }

    match parts[0] {
        "new" if parts.len() > 1 => ("new local session".into(), CommandAction::NewLocal(parts[1].into())),
        "ssh" if parts.len() > 1 => ("connecting ssh...".into(), CommandAction::NewSSH(parts[1].into())),
        "quit" | "exit" => ("bye".into(), CommandAction::Quit),
        _ => ("unknown command\n".into(), CommandAction::None),
    }
}