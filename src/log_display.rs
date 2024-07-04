use std::path::PathBuf;
use eframe::egui;

pub struct LogDisplay {
    messages: Vec<String>,
    failed_deletions: Vec<FailedDeletionFile>,
}

impl LogDisplay {
    pub fn new() -> Self {
        Self {
            messages : Vec::new(),
            failed_deletions: Vec::new(),
        }
    }

    pub fn log(&mut self, message: &String) {
        self.messages.push(message.to_string());
    }

    pub fn log_failed_deletion(&mut self, path: PathBuf, error_message: String) {
        self.failed_deletions.push(FailedDeletionFile::new(path, error_message));
    }


    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Grid::new("failed_deletion_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.label("File Name");
                ui.label("Error Message");

                for failed_file in &self.failed_deletions {
                    ui.end_row();
                    ui.label(failed_file.path.file_name().unwrap_or_default().to_string_lossy());
                    ui.label(&failed_file.error_message);
                }
            });
    }



    pub fn clear(&mut self) {
        self.messages.clear();
        self.failed_deletions.clear();
    }
}




pub struct FailedDeletionFile {
    pub path: PathBuf,
    pub error_message: String,
}

impl FailedDeletionFile {
    pub fn new(path: PathBuf, error_message: String) -> Self {
        Self { path, error_message }
    }
}
