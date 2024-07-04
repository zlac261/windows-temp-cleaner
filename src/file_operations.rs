use std::fs;
use std::path::Path;
use std::env;

pub fn clear_temp_files() -> (usize, u64, Vec<String>) {
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

fn delete_files_in_directory(dir: &Path, files_deleted: &mut usize, bytes_freed: &mut u64, failed_deletions: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        if let Ok(metadata) = entry.metadata() {
                            *files_deleted += 1;
                            *bytes_freed += metadata.len();
                        }
                    },
                    Err(e) => {
                        failed_deletions.push(format!("{}: {}", path.display(), e));
                    }
                }
            } else if path.is_dir() {
                delete_files_in_directory(&path, files_deleted, bytes_freed, failed_deletions);
                if let Err(e) = fs::remove_dir(&path) {
                    failed_deletions.push(format!("{}: {}", path.display(), e));
                }
            }
        }
    }
}