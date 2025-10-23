use eframe::egui;

pub mod gate;
mod paser;
pub mod repl;

mod editor;

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "digicir",
        native_options,
        Box::new(|cc| Ok(Box::new(crate::app::repl::Repl::new(cc)))),
    )
}