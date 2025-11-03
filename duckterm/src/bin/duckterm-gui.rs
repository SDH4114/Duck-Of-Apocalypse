fn main() {
    if let Err(err) = duckterm::gui::run() {
        eprintln!("GUI error: {err}");
        std::process::exit(1);
    }
}