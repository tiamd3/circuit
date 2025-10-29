use eframe::egui;
use crate::circuit::Circuit;
use crate::signal::Signal;

pub mod gate;
mod paser;
pub mod repl;

mod editor;
mod file;

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

pub fn run_circuit(circuit: Circuit<Signal>) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "digicir",
        native_options,
        Box::new(|cc| Ok(
            Box::new(crate::app::repl::Repl::new_with_circuit(cc, circuit)))),
    )
}
#[cfg(test)]
mod tests {
    use crate::gate::GateType;
    use super::*;

    #[test]
    fn test() {

    }
}
