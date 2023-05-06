use std::fs::{Metadata, Permissions};
use std::os::macos::fs::MetadataExt;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct File {
    pub is_dir: bool,
    pub file_mode: Permissions,
    pub number_of_links: u32,
    pub owner_name: String,
    pub group_name: String,
    pub number_of_bytes: u64,
    pub last_modified: SystemTime,
    pub path_name: String,
}

impl File {
    pub fn new(path: PathBuf, metadata: Metadata) -> File {
        File {
            is_dir: metadata.is_dir(),
            file_mode: metadata.permissions(),
            number_of_links: 0,
            owner_name: metadata.st_gid().to_string(),
            group_name: metadata.st_gid().to_string(),
            number_of_bytes: metadata.len(),
            last_modified: metadata.modified().unwrap(),
            path_name: path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        }
    }
}
