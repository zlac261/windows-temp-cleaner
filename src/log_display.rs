use std::fs;
use std::path::{Path, PathBuf};
use eframe::egui;
use crate::file_type::FileType;

#[derive(PartialEq, Clone, Copy)]
pub enum SortCriteria {
    FileName,
    ErrorMessage,
    FileType,
    Size,
}

///
/// Struct responsible for displaying the files that the system failed to delete.
pub struct LogDisplay {
    failed_deletions: Vec<FailedDeletionFile>,
    sort_by: SortCriteria,
    ascending: bool,
}

impl LogDisplay {
    pub fn new() -> Self {
        Self {
            failed_deletions: Vec::new(),
            sort_by: SortCriteria::FileName,
            ascending: true,
        }
    }

    pub fn log_failed_deletion(&mut self, path: PathBuf, error_message: String) {
        self.failed_deletions.push(FailedDeletionFile::from_path(path, error_message));
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.sort_failed_deletions();

        egui::Grid::new("failed_deletion_grid")
            .striped(true)
            .show(ui, |ui| {
                self.sortable_header(ui, "File Name", SortCriteria::FileName);
                self.sortable_header(ui, "Error Message", SortCriteria::ErrorMessage);
                self.sortable_header(ui, "Type", SortCriteria::FileType);
                self.sortable_header(ui, "Size", SortCriteria::Size);
                ui.end_row();

                for failed_file in &self.failed_deletions {
                    ui.label(failed_file.path.file_name().unwrap_or_default().to_string_lossy());
                    ui.label(&failed_file.error_message);
                    ui.label(failed_file.file_type.as_str());
                    ui.label(format!("{:.2} MB",failed_file.size as f64 / 1_000_000.0));
                    ui.end_row();
                }
            });
    }

    fn sortable_header(&mut self, ui: &mut egui::Ui, label: &str, criteria: SortCriteria) {
        let text = if self.sort_by == criteria {
            format!("{} {}", label, if self.ascending { "(Asc)" } else { "(Desc)" })
        } else {
            label.to_string()
        };

        if ui.link(text).clicked() {
            if self.sort_by == criteria {
                self.ascending = !self.ascending;
            } else {
                self.sort_by = criteria;
                self.ascending = true;
            }
        }
    }

    fn sort_failed_deletions(&mut self) {
        self.failed_deletions.sort_by(|a, b| {
            let cmp = match self.sort_by {
                SortCriteria::FileName => a.path.file_name().cmp(&b.path.file_name()),
                SortCriteria::ErrorMessage => a.error_message.cmp(&b.error_message),
                SortCriteria::FileType => a.file_type.as_str().cmp(b.file_type.as_str()),
                SortCriteria::Size => a.size.cmp(&b.size),
            };
            if self.ascending { cmp } else { cmp.reverse() }
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
        return FailedDeletionFile::new(path,error_message, file_type, size);
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