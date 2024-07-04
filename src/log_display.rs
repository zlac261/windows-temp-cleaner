use std::path::PathBuf;
use eframe::egui;

///
/// Struct responsible for displaying the files that the system failed to delete.
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
        let file_type = if path.is_dir() {
            FileType::Directory
        } else{
            FileType::File
        };

        let size = match path.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        };


        self.failed_deletions.push(FailedDeletionFile::new(path, error_message, file_type, size));
    }


    pub fn show(&self, ui: &mut egui::Ui) {
        egui::Grid::new("failed_deletion_grid")
            .striped(true)
            .show(ui, |ui| {
                ui.label("File Name");
                ui.label("Error Message");
                ui.label("Type");
                ui.label("Size");

                for failed_file in &self.failed_deletions {
                    ui.end_row();
                    ui.label(failed_file.path.file_name().unwrap_or_default().to_string_lossy());
                    ui.label(&failed_file.error_message);
                    ui.label(failed_file.file_type.as_str());
                    ui.label(format!("{:.2} MB",failed_file.size as f64 / 1_000_000.0));
                }
            });
    }


    pub fn clear(&mut self) {
        self.messages.clear();
        self.failed_deletions.clear();
    }
}



///
/// Struct that holds the information about each file that failed to be deleted.
pub struct FailedDeletionFile {
    pub path: PathBuf,
    pub error_message: String,
    pub file_type: FileType,
    pub size: u64,
}

impl FailedDeletionFile {
    pub fn new(path: PathBuf, error_message: String, file_type: FileType, size: u64) -> Self {
        Self {
            path,
            error_message,
            file_type,
            size
        }
    }
}



pub enum FileType {
    File,
    Directory,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::File => "File",
            FileType::Directory => "Directory",
        }
    }
}