use crate::*;
use colored::{ColoredString, Colorize};
use std::fs::ReadDir;
use std::path::Path;
use std::{env, fs};

// Assembles the vector returned in create_files_vector() by filling each File object with the
// given metadata.
fn insert_path_in_vector(paths: ReadDir, files: &mut Vec<File>) {
    for path in paths {
        match path {
            Ok(path) => {
                let metadata = path.metadata().unwrap();
                files.push(File::new(path.path(), metadata));
            }
            Err(error_message) => println!("{}", error_message),
        }
    }
}

// Returns a simple vector of File objects based on the given path.
fn create_files_vector(paths: ReadDir) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    insert_path_in_vector(paths, &mut files);
    alphabetically_rank_files(&mut files);

    return files;
}

fn handle_single_arguments(target_path: &str) {
    if is_file(target_path) {
        println!("{}", target_path);
    } else {
        match one_argument(target_path) {
            Ok(files) => simple_print(files),
            Err(error_message) => println!("{}", error_message),
        }
    }
}

// Handle commands with multiple files or directories to list, but without any parameter.
fn handle_folders(args: Vec<String>, multiple_arguments: bool) {
    let counter = 0;

    if args[0].chars().nth(0).unwrap() != '-' {
        if args.len() > 1 || multiple_arguments == true {
            handle_multiple_arguments(args);
        } else {
            handle_single_arguments(&args[counter]);
        }
    } else {
    }
}

// Iterates through all the command's arguments to print them one by one according to the
// formatting of the "ls" command.
fn handle_multiple_arguments(args: Vec<String>) {
    let mut counter = 0;

    while counter != args.len() {
        if is_file(&args[counter]) {
            println!("{}", &args[counter]);
            if counter != args.len() - 1 {
                println!();
            }
        } else {
            match one_argument(&args[counter]) {
                Ok(files) => {
                    println!("{}:", &args[counter]);
                    simple_print(files);
                    if counter != args.len() - 1 {
                        println!("\n");
                    }
                }
                Err(error_message) => println!("{}", error_message),
            }
        }
        counter += 1;
    }
}

// Handles both commands with multiple arguments but without parameters, and vice-versa.
pub fn handle_command() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    handle_unexisting_files(&mut args);
    let empty_single_files = handle_single_files(&mut args);

    if !args.is_empty() {
        if empty_single_files == true {
            handle_folders(args, false);
        } else {
            println!();
            handle_folders(args, true);
        }
    }
}

// Directories and files are colored differently, which is why this function is needed.
fn color_print(file: &File) -> ColoredString {
    if file.is_dir == true {
        return format!("{}", file.path_name).cyan().bold();
    } else {
        return format!("{}", file.path_name).white();
    }
}

// The function used when all that is needed is to output the files, without information about
// them.
pub fn simple_print(mut files: Vec<File>) {
    if files.len() == 1 {
        print!("{}", color_print(&files[0]));
        return;
    }
    // The terminal width is necessary to find how many columns are needed, see get_matrix_size().
    let terminal_width = match get_terminal_width() {
        Ok(terminal_width) => terminal_width,
        Err(error_message) => {
            println!("{}", error_message);
            return;
        }
    };

    // Remove all the files where the name starts with a dot.
    files.retain(|file| !file.path_name.starts_with('.'));
    let column_length = get_column_length(&files);
    let (number_of_rows, number_of_columns) =
        get_matrix_size(files.len(), terminal_width as usize, column_length);

    let file_matrix = assemble_file_matrix(number_of_columns, number_of_rows, files);
    transpose_print(file_matrix, column_length);
}

// Returns the size of the file matrix in terms of the number of rows and columns.
pub fn get_matrix_size(
    number_of_files: usize,
    terminal_width: usize,
    column_length: usize,
) -> (usize, usize) {
    // We first divide the width of the terminal the command is being used in, by the length
    // of the columns (which is the length of the longest file name + 1 (for the spacing), see
    // get_column_length()).
    let initial_number_of_columns = (terminal_width) / column_length;
    // Based on the initial estimate of columns, we can find the number of rows.
    let mut number_of_rows = number_of_files / initial_number_of_columns;
    // If the division had a rest, we need to add an additional row.
    if number_of_files % initial_number_of_columns != 0 {
        number_of_rows += 1;
    }

    // If the division had a rest, we need to add an additional column.
    let leftover_files_column = if number_of_files % (number_of_rows) != 0 {
        1
    } else {
        0
    };

    // We can now find the exact right amount of needed columns.
    let number_of_columns = number_of_files / number_of_rows + leftover_files_column;

    (number_of_rows, number_of_columns)
}

// Creates a 2D vector of files, based on the 1D "files" vector. The rows and columns are inverted,
// which is why they are first tranposed before being printed in transpose_print().
fn assemble_file_matrix(
    number_of_columns: usize,
    number_of_rows: usize,
    files: Vec<File>,
) -> Vec<Vec<File>> {
    let mut counter = 0;
    let mut file_matrix: Vec<Vec<File>> = Vec::new();

    for _ in 0..number_of_columns {
        let mut row: Vec<File> = Vec::new();
        while counter < files.len() {
            row.push(files[counter].clone());
            counter += 1;
            if counter % number_of_rows == 0 {
                break;
            }
        }
        file_matrix.push(row);
    }
    return file_matrix;
}

// In order to correctly display the files, we transpose the existing 2d vector of files
// called "file_matrix" because the rows and columns are inverted.
fn transpose_print(file_matrix: Vec<Vec<File>>, column_length: usize) {
    // Find the maximum number of rows and columns.
    // Certain columns may have different sizes, so we can't just use file_matrix[0].len() as
    // length to find the number of needed columns.
    let number_of_rows = file_matrix.len();
    let number_of_columns = file_matrix.iter().map(|row| row.len()).max().unwrap_or(0);

    for column in 0..number_of_columns {
        for row in 0..number_of_rows {
            if column < file_matrix[row].len() {
                print!("{}", color_print(&file_matrix[row][column]));
                let counter = column_length - file_matrix[row][column].path_name.len();
                for _ in 0..counter {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

// Handles function calls without any parameter or multiple arguments, is also used to handle each
// argument independently.
pub fn one_argument(target_path: &str) -> Result<Vec<File>, String> {
    let path = Path::new(target_path);

    match fs::read_dir(&path) {
        Ok(path) => Ok(create_files_vector(path)),
        Err(_) => Err(format!("ls: {}: No such file or directory", target_path).to_string()),
    }
}
