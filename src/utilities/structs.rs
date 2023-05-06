use std::fs::{Metadata, Permissions};
use std::os::macos::fs::MetadataExt;
use std::time::SystemTime;

pub struct Parameters {
    pub include_dot_files: bool,
    pub long_format: bool,
    pub reverse_order: bool,
    pub recursive_listing: bool,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            include_dot_files: false,
            long_format: false,
            reverse_order: false,
            recursive_listing: false,
        }
    }
}

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
    pub fn new(path: String, metadata: Metadata) -> File {
        File {
            is_dir: metadata.is_dir(),
            file_mode: metadata.permissions(),
            number_of_links: 0,
            owner_name: metadata.st_gid().to_string(),
            group_name: metadata.st_gid().to_string(),
            number_of_bytes: metadata.len(),
            last_modified: metadata.modified().unwrap(),
            path_name: path,
        }
    }
}
