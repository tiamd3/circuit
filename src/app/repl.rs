use eframe;
use eframe::{egui, CreationContext, Frame};
use eframe::egui::Context;
use eframe::egui::UiKind::CentralPanel;
use crate::app::paser::CircuitInterpreter;
use crate::signal::{BinarySignal, Signal};

pub struct Repl {
    interpreter: CircuitInterpreter<Signal>
}

impl Repl
{
    pub fn new(cc: &CreationContext) -> Repl {
        Self {
            interpreter: CircuitInterpreter::new("".to_string())
        }
    }
}

impl eframe::App for Repl {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.label("Welcome to Circuit Repl!");
        });
    }
}
