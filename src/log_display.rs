use std::path::PathBuf;




pub struct FailedDeletionFile {
    pub path: PathBuf,
    pub error_message: String,
}

impl FailedDeletionFile {
    pub fn new(path: PathBuf, error_message: String) -> Self {
        Self { path, error_message }
    }
}
