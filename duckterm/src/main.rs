mod app;
mod backends;
mod commands;
mod gui;
mod session;

use std::env;

fn main() {
    // если запустили: ./duckterm --gui → откроется окошко
    let use_gui = env::args().any(|a| a == "--gui");

    if use_gui {
        if let Err(err) = gui::run() {
            eprintln!("GUI error: {err}");
            std::process::exit(1);
        }
    } else {
        if let Err(err) = app::run() {
            eprintln!("TUI error: {err}");
            std::process::exit(1);
        }
    }
}