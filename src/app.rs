use eframe::egui;
use crate::file_operations;

pub struct TempFileCleanerApp {
    files_deleted: usize,
    bytes_freed: u64,
    failed_deletions: Vec<String>,
    show_failed_deletions: bool,
}

impl TempFileCleanerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            files_deleted: 0,
            bytes_freed: 0,
            failed_deletions: Vec::new(),
            show_failed_deletions: false,
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
                self.failed_deletions = result.2;
            }
            ui.label(format!("Files deleted: {}", self.files_deleted));
            ui.label(format!("Bytes freed: {} MB", self.bytes_freed / 1_000_000));

            ui.checkbox(&mut self.show_failed_deletions, "Show failed deletions");
            if self.show_failed_deletions {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for failed in &self.failed_deletions {
                        ui.label(failed);
                    }
                });
            }
        });
    }
}