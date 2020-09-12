use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInfo {
    name: String,
    last_accessed: String,
    size: u64,
    path: PathBuf,
}

#[derive(Debug)]
pub struct DirInfo {
    name: String,
    last_accessed: String,
    size: u64,
    path: PathBuf,
}

pub fn make_file_info(name: String, last_accessed: String,
                      size: u64, path: PathBuf) -> FileInfo {
    FileInfo{
        name,
        last_accessed,
        size,
        path,
    }
}

pub fn make_dir_info(name: String, last_accessed: String,
                      size: u64, path: PathBuf) -> FileInfo {
    DirInfo{
        name,
        last_accessed,
        size,
        path,
    }
}
