use std::path::PathBuf;
use eframe::egui::{self, Color32};

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
        for message in &self.messages {
            ui.label(egui::RichText::new(message).color(Color32::RED));
        }

        if !self.failed_deletions.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("Failed Deletions:").color(Color32::LIGHT_GRAY));

            for failed in &self.failed_deletions {
                ui.label(egui::RichText::new(format!("{} - Error: {}", failed.path.display(), failed.error_message))
                    .color(Color32::RED));
            }
        }
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
