use eframe::egui;
use eframe::egui::{TopBottomPanel, Context, TextEdit, RichText, Ui};
use eframe::epaint::Color32;

#[derive(Debug)]
pub struct Editor {
    code_buffer: String,
}

impl Editor {
    pub fn new() -> Self {
        Self { 
            code_buffer: String::new(),
        }
    }
    
    pub fn get_code(&self) -> String {
        self.code_buffer.clone()
    }
    
    pub fn clear_code(&mut self) {
        self.code_buffer.clear();
    }
    pub fn update(&mut self, ui: &mut Ui) {
        
        ui.add(
            TextEdit::multiline(&mut self.code_buffer)
                .frame(true)
                .code_editor()
                .desired_width(f32::INFINITY)
                .desired_rows(10)
                .font(egui::FontId::new(16.0, egui::FontFamily::Monospace))
                .text_color(egui::Color32::WHITE)
                .background_color(egui::Color32::from_rgb(30, 30 , 30 ))
        );
    }
}