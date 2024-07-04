use eframe::egui::{self, Color32, Rect, Sense, Vec2};


///
/// Struct showing the progress of the file deletion.
pub struct LoadingBar {
    progress: f32,
}

impl LoadingBar {
    pub fn new() -> Self {
        Self { progress: 0.0}
    }

    pub fn set_progress (&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn increment_progress(&mut self, amount: f32) {
        self.set_progress(self.progress + amount);
    }

    pub fn is_complete(&self) -> bool {
        return self.progress >= 1.0;
    }


    pub fn show(&self, ui: &mut egui::Ui) {
        let size = Vec2::new(ui.available_width(), 20.0);
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let visuals = ui.style().visuals.clone();
        let bar_color =  self.colour_shade();
        let bg_color = visuals.extreme_bg_color;
        let percentage = (self.progress * 100.0) as i32;

        ui.painter().rect_filled(rect, 0.0, bg_color);
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min, egui::vec2(rect.width() * self.progress, rect.height())),
            0.0,
            bar_color,
        );

        let text = format!("{}%", percentage);
        let text_color = Color32::BLACK;
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(14.0),
            text_color,
        );
    }

    fn colour_shade(&self) -> Color32 {
        let red = 255.0 * (1.0 - self.progress);
        let green = 255.0 * self.progress;
        return Color32::from_rgb(red as u8, green as u8, 0);
    }
}

