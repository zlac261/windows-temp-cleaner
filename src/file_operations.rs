use std::fs;
use std::path::Path;
use std::env;

pub fn clear_temp_files() -> (usize, u64) {
    let mut files_deleted = 0;
    let mut bytes_freed = 0;


    let temp_dirs = vec![
        env::temp_dir(),
        Path::new("C:\\Windows\\Temp").to_path_buf(),
    ];

    for dir in temp_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let file_size = metadata.len();
                    if fs::remove_file(entry.path()).is_ok() {
                        files_deleted += 1;
                        bytes_freed += file_size;
                    }
                }
            }
        }
    }

    (files_deleted, bytes_freed)
}