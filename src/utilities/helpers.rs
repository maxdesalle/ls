use crate::*;
use std::fs::read_link;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use terminal_size::{terminal_size, Width};

// Orders a vector of File objects in descending order based on the last modified date.
pub fn rank_files_by_last_modified_date(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| b.last_modified.partial_cmp(&a.last_modified).unwrap());
}

// Orders a vector of File objects in ascending order based on the last modified date.
pub fn reverse_rank_files_by_last_modified_date(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| a.last_modified.partial_cmp(&b.last_modified).unwrap());
}

// Orders a vector of File objects in ascending order based on the last modified date.
pub fn rank_path_by_last_modified_date(files: &mut Vec<String>) {
    files.sort_unstable_by(|a, b| {
        (Path::new(b).metadata().unwrap().modified().unwrap())
            .partial_cmp(&Path::new(&a).metadata().unwrap().modified().unwrap())
            .unwrap()
    });
}

// Orders a vector of File objects in ascending order based on the last modified date.
pub fn reverse_rank_path_by_last_modified_date(files: &mut Vec<String>) {
    files.sort_unstable_by(|a, b| {
        (Path::new(a).metadata().unwrap().modified().unwrap())
            .partial_cmp(&Path::new(&b).metadata().unwrap().modified().unwrap())
            .unwrap()
    });
}

// Orders a vector of File objects alphabetically based on their path_name variable.
pub fn alphabetically_rank_files(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| a.path_name.partial_cmp(&b.path_name).unwrap());
}

// Orders a vector of File objects alphabetically in reverse based on their path_name variable.
pub fn reverse_alphabetically_rank_files(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| b.path_name.partial_cmp(&a.path_name).unwrap());
}

// Orders a vector of strings alphabetically.
pub fn alphabetically_rank_strings(strings: &mut Vec<String>) {
    strings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}

// Orders a vector of strings alphabetically in reverse.
pub fn reverse_alphabetically_rank_strings(strings: &mut Vec<String>) {
    strings.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
}

// Alphabetically ranks a vector of PathBuf objects.
pub fn alphabetically_rank_path_bufs(paths: &mut Vec<PathBuf>) {
    paths.sort_unstable_by(|a, b| get_path_name(a).partial_cmp(&get_path_name(b)).unwrap());
}

// Alphabetically ranks a vector in reverse of PathBuf objects.
pub fn reverse_alphabetically_rank_path_bufs(paths: &mut Vec<PathBuf>) {
    paths.sort_unstable_by(|a, b| get_path_name(b).partial_cmp(&get_path_name(a)).unwrap());
}

// Ranks a vector of PathBuf objects by their last modification date.
pub fn rank_path_bufs_by_last_modified_date(paths: &mut Vec<PathBuf>) {
    paths.sort_unstable_by(|a, b| {
        (b.metadata().unwrap().modified().unwrap())
            .partial_cmp(&a.metadata().unwrap().modified().unwrap())
            .unwrap()
    });
}

// Ranks a vector of PathBuf objects in reverse by their last modification date.
pub fn reverse_rank_path_bufs_by_last_modified_date(paths: &mut Vec<PathBuf>) {
    paths.sort_unstable_by(|a, b| {
        (a.metadata().unwrap().modified().unwrap())
            .partial_cmp(&b.metadata().unwrap().modified().unwrap())
            .unwrap()
    });
}

// Checks if the path points to a file or a directory.
pub fn is_file(target_path: &str) -> bool {
    let path = Path::new(target_path);

    path.is_file()
}

// Checks if the path points to a file or a directory.
pub fn file_exists(target_path: &str) -> bool {
    let path = Path::new(target_path);

    path.exists()
}

// Return the position in the vector of the String to look for.
pub fn return_index_for_object(args: &mut Vec<String>, object_to_find: &String) -> usize {
    args.iter().position(|x| *x == *object_to_find).unwrap()
}

// Gets the width of the terminal, so the number of columns the "ls" command outputs can be
// calculated.
pub fn get_terminal_width() -> Result<u16, String> {
    let size = terminal_size();

    match size {
        Some((Width(w), _)) => Ok(w),
        None => Err(format!("Unable to get terminal size")),
    }
}

// Converts a vector of PathBuf objects to a vector of String objects.
pub fn convert_path_buf_vector_to_string_vector(pathbufs: &Vec<PathBuf>) -> Vec<String> {
    let filenames: Vec<String> = pathbufs
        .iter()
        .filter_map(|path| path.to_str())
        .map(|str| str.to_owned())
        .collect();

    filenames
}

// Converts a vector of String objects to a vector of File structs.
pub fn convert_string_vector_to_file_vector(strings: Vec<String>) -> Vec<File> {
    let files: Vec<File> = strings
        .into_iter()
        .map(|string| {
            let path = Path::new(&string);
            File::new(
                path.file_name().unwrap().to_str().unwrap().to_owned(),
                path.metadata().unwrap(),
                check_extended_attributes(&path),
            )
        })
        .collect();

    files
}

// Returns the length of the longest path name in the "files" vector, adding 1 for spacing.
pub fn get_column_length_single_files(files: &Vec<String>) -> usize {
    files.iter().max_by_key(|file| file.len()).unwrap().len() + 1
}

// Check whether or not the given file is executable.
pub fn is_executable(file: &File) -> bool {
    file.file_mode.mode() & 0o111 != 0
}

// Returns the file name the symbolic link is pointing towards. Used for long format printing.
pub fn get_symbolic_link(file: &File) -> String {
    read_link(&file.path_name)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// Returns the length of the longest path name in the "files" vector, adding 1 for spacing.
pub fn get_column_length(files: &Vec<File>) -> usize {
    files
        .iter()
        .max_by_key(|file| file.path_name.len())
        .unwrap()
        .path_name
        .len()
        + 1
}

// Returns the file name of a given PathBuf object
pub fn get_path_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}
