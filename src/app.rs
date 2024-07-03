use eframe::egui;
use crate::file_operations;

pub struct TempFileCleanerApp {
    files_deleted: usize,
    bytes_freed: u64,
}

impl TempFileCleanerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            files_deleted: 0,
            bytes_freed: 0,
        }
    }
}

impl eframe::App for TempFileCleanerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Windows Temp Cleaner");
            if ui.button("Clear Temp Files").clicked() {
                let result = file_operations::clear_temp_files();
                self.files_deleted = result.0;
                self.bytes_freed = result.1;
            }
            ui.label(format!("Files deleted: {}", self.files_deleted));
            ui.label(format!("Bytes freed: {} MB", self.bytes_freed / 1_000_000));
        });
    }
}