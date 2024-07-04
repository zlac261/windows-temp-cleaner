use eframe::egui;
use crate::file_operations;
use crate::loading_bar::LoadingBar;
use crate::log_display::LogDisplay;

pub struct TempFileCleanerApp {
    loading_bar: LoadingBar,
    log_display: LogDisplay,
    files_deleted: usize,
    bytes_freed: u64,
    show_failed_deletions: bool,
    operation_result: Option<String>,
    is_cleaning: bool,
}

impl TempFileCleanerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            loading_bar: LoadingBar::new(),
            files_deleted: 0,
            bytes_freed: 0,
            log_display: LogDisplay::new(),
            show_failed_deletions: false,
            operation_result: None,
            is_cleaning: false,
        }
    }

    pub fn log(&mut self, message: &String) {
        self.log_display.log(message);
    }

}

impl eframe::App for TempFileCleanerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::containers::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .inner_margin(20.0)
            .rounding(10.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.heading(egui::RichText::new("Windows Temp Cleaner")
                .size(24.0)
                .color(egui::Color32::from_rgb(200, 200, 255)));
            ui.add_space(20.0);

            if !self.is_cleaning {
                if ui.add_sized(
                    [200.0, 40.0],
                    egui::Button::new(
                        egui::RichText::new("Clear Temp Files")
                            .color(egui::Color32::BLACK)
                    )
                        .fill(egui::Color32::from_rgb(100, 200, 100)),
                ).clicked() {
                    self.is_cleaning = true;
                    self.loading_bar.set_progress(0.0);
                    self.log_display.clear();
                }
            } else {
                self.loading_bar.increment_progress(0.01);
                if self.loading_bar.is_complete() {
                    self.is_cleaning = false;
                    let result = file_operations::clear_temp_files();
                    self.files_deleted = result.0;
                    self.bytes_freed = result.1;
                    let failed_files = result.2;
                    self.operation_result = Some(if failed_files.is_empty() {
                        "Temp files cleared successfully!".to_string()
                    } else {
                        "Some files could not be deleted.".to_string()
                    });

                    let message = format!("Operation result: {:?}", self.operation_result);
                    self.log(&message);
                    for failure in &failed_files {
                        self.log_display.log_failed_deletion(failure.path.clone(), failure.error_message.clone());
                    }
                }

                self.loading_bar.show(ui);
            }

            ui.add_space(20.0);

            egui::Grid::new("info_grid").num_columns(2).spacing([40.0, 4.0]).show(ui, |ui| {
                ui.label(egui::RichText::new("Files Deleted:").color(egui::Color32::LIGHT_GRAY));
                ui.label(egui::RichText::new(self.files_deleted.to_string()).color(egui::Color32::WHITE));
                ui.end_row();

                ui.label(egui::RichText::new("Space Freed:").color(egui::Color32::LIGHT_GRAY));
                ui.label(egui::RichText::new(format!("{:.2} MB", self.bytes_freed as f64 / 1_000_000.0)).color(egui::Color32::WHITE));
                ui.end_row();
            });

            ui.add_space(10.0);

            if let Some(result) = &self.operation_result {
                ui.colored_label(
                    if result.contains("successfully") { egui::Color32::GREEN } else { egui::Color32::RED },
                    result,
                );
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.checkbox(&mut self.show_failed_deletions, egui::RichText::new("Show failed deletions").color(egui::Color32::LIGHT_GRAY));

            if self.show_failed_deletions {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    self.log_display.show(ui);
                });
            }
        });
    }
}