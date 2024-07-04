pub enum FileType {
    File, // Default type
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
