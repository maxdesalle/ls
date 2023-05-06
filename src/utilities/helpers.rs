use crate::File;
use std::path::Path;
use terminal_size::{terminal_size, Width};

// Orders a vector of File objects alphabetically based on their path_name variable.
pub fn alphabetically_rank_files(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| a.path_name.partial_cmp(&b.path_name).unwrap());
}

// Orders a vector of strings alphabetically.
pub fn alphabetically_rank_strings(strings: &mut Vec<String>) {
    strings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
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
