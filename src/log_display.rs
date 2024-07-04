use std::fs;
use std::path::{Path, PathBuf};
use eframe::egui;
use crate::file_type::FileType;

///
/// Struct responsible for displaying the files that the system failed to delete.
pub struct LogDisplay {
    failed_deletions: Vec<FailedDeletionFile>,
}

impl LogDisplay {
    pub fn new() -> Self {
        Self {
            failed_deletions: Vec::new(),
        }
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

    pub fn from_path(path: PathBuf, error_message: String) -> Self {
        let file_type = determine_file_type(&path);
        let size = get_file_size(&path);
        Self {
            path,
            error_message,
            file_type,
            size
        }
    }
}


fn determine_file_type (path: &Path) -> FileType {
    if path.is_symlink(){
        return FileType::Symlink;
    } else if path.is_dir(){
        return FileType::Directory;
    } else if let Ok(metadata) = path.metadata() {
        if metadata.is_file(){
            if metadata.permissions().readonly() {
                return FileType::ReadOnlyFile;
            }
        }
    }
    return FileType::File;
}

fn get_file_size(path: &Path) -> u64 {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.len()
    } else {
        0
    }
}