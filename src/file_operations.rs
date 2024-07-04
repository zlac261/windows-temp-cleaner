use std::fs;
use std::path::Path;
use std::env;
use crate::log_display::{FailedDeletionFile, FileType};

pub fn clear_temp_files() -> (usize, u64, Vec<FailedDeletionFile>) {
    let mut files_deleted = 0;
    let mut bytes_freed = 0;
    let mut failed_deletions = Vec::new();

    let temp_dirs = vec![
        env::temp_dir(),
        Path::new("C:\\Windows\\Temp").to_path_buf(),
    ];

    for dir in temp_dirs {
        delete_files_in_directory(&dir, &mut files_deleted, &mut bytes_freed, &mut failed_deletions);
    }

    (files_deleted, bytes_freed, failed_deletions)
}

fn delete_files_in_directory(dir: &Path, files_deleted: &mut usize, bytes_freed: &mut u64, failed_deletions: &mut Vec<FailedDeletionFile>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_type = determine_file_type(&path);
            if path.is_file() {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        if let Ok(metadata) = entry.metadata() {
                            *files_deleted += 1;
                            *bytes_freed += metadata.len();
                        }
                    },
                    Err(e) => {
                        let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                        failed_deletions.push(FailedDeletionFile::new(path.clone(), e.to_string(), file_type, size));
                    }
                }
            } else if path.is_dir() {
                delete_files_in_directory(&path, files_deleted, bytes_freed, failed_deletions);
                if let Err(e) = fs::remove_dir(&path) {
                    let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                    failed_deletions.push(FailedDeletionFile::new(path.clone(), e.to_string(), file_type, size));
                }
            }
        }
    }
}


fn determine_file_type (path: &Path) -> FileType {
    return if path.is_dir() {
        FileType::Directory
    } else {
        FileType::File
    }
}