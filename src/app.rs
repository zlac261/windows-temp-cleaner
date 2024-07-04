use eframe::egui;
use crate::file_operations;

pub struct TempFileCleanerApp {
    files_deleted: usize,
    bytes_freed: u64,
    failed_deletions: Vec<String>,
    show_failed_deletions: bool,
    operation_result: Option<String>,
    is_cleaning: bool,
    progress: f32,
}

impl TempFileCleanerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            files_deleted: 0,
            bytes_freed: 0,
            failed_deletions: Vec::new(),
            show_failed_deletions: false,
            operation_result: None,
            is_cleaning: false,
            progress: 0.0,
        }
    }
}

impl eframe::App for TempFileCleanerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::containers::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .inner_margin(20.0)
            .rounding(10.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.heading(egui::RichText::new("Windows Temp Cleaner").size(24.0).color(egui::Color32::from_rgb(200, 200, 255)));
            ui.add_space(20.0);

            if !self.is_cleaning {
                if ui.add(egui::Button::new("Clear Temp Files").min_size(egui::vec2(200.0, 40.0))).clicked() {
                    self.is_cleaning = true;
                    self.progress = 0.0;
                    // In a real application, you'd start the cleaning process here
                }
            } else {
                // Simulating progress
                self.progress += 0.01;
                if self.progress >= 1.0 {
                    self.is_cleaning = false;
                    let result = file_operations::clear_temp_files();
                    self.files_deleted = result.0;
                    self.bytes_freed = result.1;
                    self.failed_deletions = result.2;
                    self.operation_result = Some(if self.failed_deletions.is_empty() {
                        "Temp files cleared successfully!".to_string()
                    } else {
                        "Some files could not be deleted.".to_string()
                    });
                }
                ui.add(egui::ProgressBar::new(self.progress).show_percentage());
            }

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Files Deleted:");
                ui.label(self.files_deleted.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Bytes Freed:");
                ui.label(format!("{:.2} MB", self.bytes_freed as f64 / 1_000_000.0));
            });

            if let Some(result) = &self.operation_result {
                ui.label(result);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

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