use std::fmt::format;
use eframe;
use eframe::{egui, CreationContext, Frame};
use eframe::egui::{Color32, Context, Key, RichText, TextEdit, Ui};
use macroquad::prelude::is_key_pressed;
use crate::app::editor::Editor;
use crate::app::paser::{CircuitInterpreter, DigicirParser, Interpreter};
use crate::circuit::Circuit;
use crate::signal::{BinarySignal, Signal};

#[derive(Debug)]
pub struct Repl {
    circuit: Circuit<Signal>,
    editor: Editor,
    message: ParseMessage,
    truth_table: TruthTable,
}

#[derive(Debug)]
pub struct ParseMessage {
    flag: bool,
    res_flag: bool,
    message: String,
}

#[derive(Debug)]
pub struct TruthTable {
    flag: bool,
    data: Vec<(Vec<Signal>, Vec<Signal>)>,
    strings: Vec<String>,
}

impl ParseMessage {
    pub fn new() -> Self {
        Self { flag: false, res_flag: false, message: String::new() }
    }

    pub fn update(&mut self, s: String, res_flag: bool) {
        self.message = s;
        self.flag = true;
        self.res_flag = res_flag;
    }

    pub fn draw(&self, ui: &mut Ui) {
        if self.flag && !self.res_flag {
            if !self.message.is_empty() {
                ui.separator();
                ui.label(RichText::new("Parse Result: ")
                    .color(Color32::WHITE)
                    .size(18.0));
                ui.label(RichText::new(&self.message)
                    .color(Color32::WHITE)
                    .size(18.0));
            }
        }
    }
}

impl TruthTable {
    pub fn new() -> Self {
        Self { flag: false, data: Vec::new(), strings: Vec::new() }
    }

    pub fn update(&mut self, circuit: &mut Circuit<Signal>, output: &[usize]) {
        let mut new_data= vec![];
        let input_size = circuit.get_input();
        let max = 2u32.pow(input_size as u32);
        for i in 0..max {
            let mut j = i;
            let mut vec = Vec::with_capacity(input_size);
            for _ in 0..input_size {
                let v = if j % 2 == 0 { false } else { true };
                vec.push(v);
                j /= 2;
            }
            vec.reverse();
            let input_values = vec.iter()
                .map(|b| Signal::from_bool(*b))
                .collect::<Vec<Signal>>();
            circuit.execute_gates(&input_values);
            let output = circuit.clone_signals(output).clone();
            new_data.push((input_values, output));
        }
        let mut strings = Vec::new();
        for (input_values, output) in &new_data {
            strings.push(format!("{:?}", input_values) + "  |  " + &format!("{:?}", output));
        }
        self.flag = true;
        self.data = new_data;
        self.strings = strings;
    }

    pub fn draw(&self, ui: &mut Ui) {
        if self.flag {
            ui.separator();
            ui.label(RichText::new("Truth Table:").size(18.0).color(Color32::WHITE));
            ui.vertical(|ui| {
                for string in &self.strings {
                    ui.label(RichText::new(string).size(18.0).color(Color32::WHITE));
                }
            });
        }

    }
}

impl Repl
{
    pub fn new(cc: &CreationContext) -> Repl {
        Self {
            editor: Editor::new(),
            circuit: Circuit::new(),
            message: ParseMessage::new(),
            truth_table: TruthTable::new(),
        }
    }
    
    pub fn get_mut_circuit(&mut self) -> &mut Circuit<Signal> {
        &mut self.circuit
    }
    
    pub fn get_circuit(&self) -> &Circuit<Signal> {
        &self.circuit
    }

    pub fn get_message(&self) -> &ParseMessage {
        &self.message
    }

    pub fn get_truth_table(&self) -> &TruthTable {
        &self.truth_table
    }

    pub fn get_truth_table_mut(&mut self) -> &mut TruthTable {
        &mut self.truth_table
    }

    pub fn draw_gates(&self, ui: &mut Ui) {
        if !self.circuit.get_gates().is_empty() {
            ui.separator();
            ui.vertical(|ui|{
                for gate in self.circuit.get_gates() {

                    ui.label(RichText::new(
                        &format!("{:?}: {:?} > {:?}",
                                 gate.get_type(), gate.get_input(), gate.get_output()
                        ))
                        .color(Color32::WHITE)
                        .size(18.0));
                }
            });
        }
    }
    pub fn draw_circuit_state(&self, ui: &mut Ui) {

        ui.label(
            RichText::new(&format!("input: {}", self.circuit.get_input()))
                .color(Color32::WHITE)
                .size(18.0)
        );
        self.draw_gates(ui);
        self.message.draw(ui);
        self.truth_table.draw(ui);
    }

    pub fn update_truth_table(&mut self, output: &[usize]) {
        self.truth_table.update(&mut self.circuit, output);
    }

    pub fn code_save_action(&mut self, ui: &mut Ui) {
        let code = self.editor.get_code();
        let res = Interpreter::parse(self, &code);
        let (res_flag, parse_message) = match res {
            Ok(_) => (true, "Ok".to_string()),
            Err(e) => (false, e.to_string()),
        };
        self.message.update(parse_message, res_flag);
        self.editor.clear_code();
    }
    pub fn update_code_editor(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            if ui.button(
                RichText::new("save")
                    .size(18.0)
                    .color(Color32::WHITE)
            ).clicked() {
               self.code_save_action(ui);
            }

            if ui.button(
                RichText::new("clear")
                    .size(18.0)
                    .color(Color32::WHITE)
            ).clicked() {
                self.editor.clear_code();
            }
        });

        let save_event = ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S));
        if save_event {
            self.code_save_action(ui);
        }

        self.editor.update(ui);
    }
}

impl eframe::App for Repl {

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           self.draw_circuit_state(ui);
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            self.update_code_editor(ui, ctx);
        });

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_convert() {
    }
}
