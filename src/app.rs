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
    info_icon: egui::TextureHandle,
    warning_icon: egui::TextureHandle,
}

impl TempFileCleanerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        let icon_size = 30;
        Self {
            files_deleted: 0,
            bytes_freed: 0,
            failed_deletions: Vec::new(),
            show_failed_deletions: false,
            operation_result: None,
            is_cleaning: false,
            progress: 0.0,
            info_icon: load_texture_from_bytes(ctx, include_bytes!("../assets/info.png"), "info", icon_size),
            warning_icon: load_texture_from_bytes(ctx, include_bytes!("../assets/warning.png"), "warning", icon_size),
        }
    }
}

fn resize_image(image_bytes: &[u8], new_size: u32) -> egui::ColorImage {
    let image = image::load_from_memory(image_bytes).unwrap();
    let resized = image.resize(new_size, new_size, image::imageops::FilterType::Lanczos3);
    let size = [new_size as _, new_size as _];
    let image_buffer = resized.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}

fn load_texture_from_bytes(ctx: &egui::Context, image_bytes: &[u8], name: &str, size: u32) -> egui::TextureHandle {
    let color_image = resize_image(image_bytes, size);
    ctx.load_texture(name, color_image, egui::TextureOptions::default())
}

impl eframe::App for TempFileCleanerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dark_bg = egui::Color32::from_rgb(30, 30, 30);
        let frame = egui::containers::Frame::none()
            .fill(dark_bg)
            .inner_margin(20.0)
            .rounding(10.0);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.heading(egui::RichText::new("Windows Temp Cleaner")
                .size(24.0)
                .color(egui::Color32::from_rgb(200, 200, 255)));
            ui.add_space(20.0);

            if !self.is_cleaning {
                ui.horizontal(|ui| {
                    if ui.add_sized(
                        [180.0, 40.0],
                        egui::Button::new(
                            egui::RichText::new("Clear Temp Files")
                                .color(egui::Color32::BLACK)
                        )
                            .fill(egui::Color32::from_rgb(100, 200, 100))
                    ).clicked() {
                        self.is_cleaning = true;
                        self.progress = 0.0;
                    }
                });
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

                // Custom progress bar with centered percentage
                let desired_size = egui::vec2(ui.available_width(), 20.0);
                let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

                let visuals = ui.style().visuals.clone();
                let bar_color = egui::Color32::from_rgb(100, 200, 100);
                let bg_color = visuals.extreme_bg_color;
                let percentage = (self.progress * 100.0) as i32;

                ui.painter().rect_filled(rect, 0.0, bg_color);
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * self.progress, rect.height())),
                    0.0,
                    bar_color,
                );

                let text = format!("{}%", percentage);
                let text_color = egui::Color32::BLACK;
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(14.0),
                    text_color,
                );
            }

            ui.add_space(20.0);

            egui::Grid::new("info_grid").num_columns(3).spacing([10.0, 4.0]).show(ui, |ui| {
                ui.image(&self.info_icon);
                ui.label(egui::RichText::new("Files Deleted:").color(egui::Color32::LIGHT_GRAY));
                ui.label(egui::RichText::new(self.files_deleted.to_string()).color(egui::Color32::WHITE));
                ui.end_row();

                ui.image(&self.warning_icon);
                ui.label(egui::RichText::new("Space Freed:").color(egui::Color32::LIGHT_GRAY));
                ui.label(egui::RichText::new(format!("{:.2} MB", self.bytes_freed as f64 / 1_000_000.0)).color(egui::Color32::WHITE));
                ui.end_row();
            });

            ui.add_space(10.0);

            if let Some(result) = &self.operation_result {
                ui.colored_label(
                    if result.contains("successfully") { egui::Color32::GREEN } else { egui::Color32::RED },
                    result
                );
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_failed_deletions, egui::RichText::new("Show failed deletions").color(egui::Color32::LIGHT_GRAY));
            });

            if self.show_failed_deletions {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for failed in &self.failed_deletions {
                        ui.label(egui::RichText::new(failed).color(egui::Color32::LIGHT_RED));
                    }
                });
            }
        });
    }
}