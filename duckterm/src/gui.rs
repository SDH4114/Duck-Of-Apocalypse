use eframe::egui;

pub struct DuckGui;

impl DuckGui {
    pub fn new() -> Self {
        Self
    }
}

impl eframe::App for DuckGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦆 DuckTerm GUI");
            ui.label("Добро пожаловать! Это графическая версия твоего терминала.");
            if ui.button("Закрыть").clicked() {
                std::process::exit(0);
            }
        });
    }
}

pub fn run() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "DuckTerm 🦆",
        native_options,
        Box::new(|_cc| Ok(Box::new(DuckGui::new()))),
    )?;
    Ok(())
}