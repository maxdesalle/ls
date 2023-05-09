use std::fs::{Metadata, Permissions};
use users::{get_group_by_gid, get_user_by_uid};

use std::time::SystemTime;

// Function to be called if the target OS is MacOS
#[cfg(target_os = "macos")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "macos")]
fn number_of_links(metadata: &Metadata) -> u64 {
    (metadata as &dyn MetadataExt).nlink()
}
#[cfg(target_os = "macos")]
fn number_of_blocks(metadata: &Metadata) -> u64 {
    (metadata as &dyn MetadataExt).blocks()
}

// Function to be called if the target OS is Linux
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "linux")]
fn number_of_links(metadata: &Metadata) {
    (metadata as &dyn MetadataExt).st_nlink()
}
#[cfg(target_os = "linux")]
fn number_of_blocks(metadata: &Metadata) {
    (metadata as &dyn MetadataExt).st_blocks()
}

fn get_username(id: u32) -> String {
    get_user_by_uid(id)
        .unwrap()
        .name()
        .to_string_lossy()
        .into_owned()
}

fn get_group(id: u32) -> String {
    get_group_by_gid(id)
        .unwrap()
        .name()
        .to_string_lossy()
        .into_owned()
}

pub struct Parameters {
    pub include_dot_files: bool,
    pub long_format: bool,
    pub reverse_order: bool,
    pub recursive_listing: bool,
    pub last_modified_order: bool,
}

impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            include_dot_files: false,
            long_format: false,
            reverse_order: false,
            recursive_listing: false,
            last_modified_order: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub is_dir: bool,
    pub is_symbolic_link: bool,
    pub file_mode: Permissions,
    pub number_of_links: u64,
    pub owner_name: String,
    pub group_name: String,
    pub number_of_bytes: u64,
    pub last_modified: SystemTime,
    pub path_name: String,
    pub extended_attributes: bool,
    pub blocks: u64,
}

impl File {
    pub fn new(path: String, metadata: Metadata, attributes: bool) -> File {
        File {
            is_dir: metadata.is_dir(),
            is_symbolic_link: metadata.is_symlink(),
            file_mode: metadata.permissions(),
            number_of_links: number_of_links(&metadata),
            owner_name: get_username(metadata.uid()),
            group_name: get_group(metadata.gid()),
            number_of_bytes: metadata.len(),
            last_modified: metadata.modified().unwrap(),
            path_name: path,
            extended_attributes: attributes,
            blocks: number_of_blocks(&metadata),
        }
    }
}
