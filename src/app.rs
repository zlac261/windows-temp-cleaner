use eframe::{CreationContext, egui};
use crate::file_operations;

pub struct TempFileCleanerApp {
    files_deleted: usize,
    bytes_freed: u64,
    failed_deletions: Vec<String>,
    show_failed_deletions: bool,
    operation_result: Option<String>,
}

impl TempFileCleanerApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            files_deleted: 0,
            bytes_freed: 0,
            failed_deletions: Vec::new(),
            show_failed_deletions: false,
            operation_result: None,
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

                self.operation_result = Some(
                    if self.failed_deletions.is_empty() {
                        "Temp files cleared successfully!".to_string()
                    } else {
                        "Some files could not be deleted.".to_string()
                    }
                );
            }

            if let Some(result) = &self.operation_result {
                ui.label(result);
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Files Deleted:");
                ui.label(self.files_deleted.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Bytes Freed:");
                ui.label(format!("{:.2} MB", self.bytes_freed as f64 / 1_000_000.0));
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.checkbox(&mut self.show_failed_deletions, "Show failed deletions");

            if self.show_failed_deletions && !self.failed_deletions.is_empty() {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for failed in &self.failed_deletions {
                        ui.label(failed);
                    }
                });
            }
        });
    }
}