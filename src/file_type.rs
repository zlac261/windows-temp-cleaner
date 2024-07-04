pub enum FileType {
    File, // Default type
    Directory,
    ReadOnlyFile,
    Symlink,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::File => "File",
            FileType::Directory => "Directory",
            FileType::ReadOnlyFile => "Read-Only",
            FileType::Symlink => "Symlink"
        }
    }
}
