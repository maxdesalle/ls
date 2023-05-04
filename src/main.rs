use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fs;
use std::fs::Metadata;
use std::fs::Permissions;
use std::fs::ReadDir;
use std::os::macos::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use terminal_size::{terminal_size, Width};

#[allow(warnings)]
#[derive(Debug, Clone)]
struct File {
    is_dir: bool,
    file_mode: Permissions,
    number_of_links: u32,
    owner_name: String,
    group_name: String,
    number_of_bytes: u64,
    last_modified: SystemTime,
    path_name: String,
}

impl File {
    fn new(path: PathBuf, metadata: Metadata) -> File {
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

// Orders a vector of File objects alphabetically based on their path_name variable.
fn alphabetically_rank_files(files: &mut Vec<File>) {
    files.sort_unstable_by(|a, b| a.path_name.partial_cmp(&b.path_name).unwrap());
}

// Orders a vector of strings alphabetically.
fn alphabetically_rank_strings(strings: &mut Vec<String>) {
    strings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}

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
fn print_no_parameter(args: Vec<String>, multiple_arguments: bool) {
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

// Checks if the path points to a file or a directory.
fn is_file(target_path: &str) -> bool {
    let path = Path::new(target_path);

    path.is_file()
}

// Checks if the path points to a file or a directory.
fn file_exists(target_path: &str) -> bool {
    let path = Path::new(target_path);

    path.exists()
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

fn collect_single_files(args: &mut Vec<String>) -> Vec<String> {
    let mut counter = 0;
    let mut single_files: Vec<String> = Vec::new();

    while counter != args.len() {
        if is_file(&args[counter]) {
            single_files.push(args[counter].to_string());
        }
        counter += 1;
    }
    counter = 0;
    while counter != single_files.len() {
        let index = args
            .iter()
            .position(|x| *x == single_files[counter])
            .unwrap();
        args.remove(index);
        counter += 1;
    }
    alphabetically_rank_strings(&mut single_files);
    single_files
}

// The function used when all that is needed is to output the files, without information about
// them.
fn simple_print_single_files(files: &Vec<String>) {
    // The terminal width is necessary to find how many columns are needed, see get_matrix_size().
    let terminal_width = match get_terminal_width() {
        Ok(terminal_width) => terminal_width,
        Err(error_message) => {
            println!("{}", error_message);
            return;
        }
    };

    let column_length = get_column_length_single_files(&files);
    let (number_of_rows, number_of_columns) =
        get_matrix_size(files.len(), terminal_width as usize, column_length);

    let file_matrix = assemble_file_matrix_single_files(number_of_columns, number_of_rows, &files);
    transpose_print_single_files(file_matrix, column_length);
}

// Creates a 2D vector of files, based on the 1D "files" vector. The rows and columns are inverted,
// which is why they are first tranposed before being printed in transpose_print().
fn assemble_file_matrix_single_files(
    number_of_columns: usize,
    number_of_rows: usize,
    files: &Vec<String>,
) -> Vec<Vec<String>> {
    let mut counter = 0;
    let mut file_matrix: Vec<Vec<String>> = Vec::new();

    for _ in 0..number_of_columns {
        let mut row: Vec<String> = Vec::new();
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
fn transpose_print_single_files(file_matrix: Vec<Vec<String>>, column_length: usize) {
    // Find the maximum number of rows and columns.
    // Certain columns may have different sizes, so we can't just use file_matrix[0].len() as
    // length to find the number of needed columns.
    let number_of_rows = file_matrix.len();
    let number_of_columns = file_matrix.iter().map(|row| row.len()).max().unwrap_or(0);

    for column in 0..number_of_columns {
        for row in 0..number_of_rows {
            if column < file_matrix[row].len() {
                print!("{}", file_matrix[row][column]);
                let counter = column_length - file_matrix[row][column].len();
                for _ in 0..counter {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

fn return_index_for_object(args: &mut Vec<String>, object_to_find: &String) -> usize {
    args.iter().position(|x| *x == *object_to_find).unwrap()
}

fn remove_unexisting_files(args: &mut Vec<String>, unexisting_files: &mut Vec<String>) {
    let mut counter = 0;

    while counter != unexisting_files.len() {
        let index = return_index_for_object(args, &unexisting_files[counter]);
        args.remove(index);
        counter += 1;
    }
}

fn find_unexisting_files(args: &mut Vec<String>) -> Vec<String> {
    let mut counter = 0;
    let mut unexisting_files: Vec<String> = Vec::new();

    while counter != args.len() {
        if !file_exists(&args[counter]) {
            unexisting_files.push(args[counter].to_string());
        }
        counter += 1;
    }

    unexisting_files
}

fn collect_unexisting_files(args: &mut Vec<String>) -> Vec<String> {
    let mut unexisting_files = find_unexisting_files(args);

    remove_unexisting_files(args, &mut unexisting_files);
    alphabetically_rank_strings(&mut unexisting_files);

    unexisting_files
}

fn simple_print_unexisting_files(unexisting_files: &Vec<String>) {
    for i in unexisting_files {
        println!("ls: {}: No such file or directory", i);
    }
}

// Handles both commands with multiple arguments but without parameters, and vice-versa.
fn handle_command() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    let unexisting_files: Vec<String> = collect_unexisting_files(&mut args);
    simple_print_unexisting_files(&unexisting_files);
    let single_files: Vec<String> = collect_single_files(&mut args);
    if !single_files.is_empty() {
        simple_print_single_files(&single_files);
    }

    if !args.is_empty() {
        if single_files.is_empty() {
            print_no_parameter(args, false);
        } else {
            println!();
            print_no_parameter(args, true);
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

// Gets the width of the terminal, so the number of columns the "ls" command outputs can be
// calculated.
fn get_terminal_width() -> Result<u16, String> {
    let size = terminal_size();

    match size {
        Some((Width(w), _)) => Ok(w),
        None => Err(format!("Unable to get terminal size")),
    }
}

// The function used when all that is needed is to output the files, without information about
// them.
fn simple_print(mut files: Vec<File>) {
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

// Returns the length of the longest path name in the "files" vector, adding 1 for spacing.
fn get_column_length_single_files(files: &Vec<String>) -> usize {
    files.iter().max_by_key(|file| file.len()).unwrap().len() + 1
}

// Returns the length of the longest path name in the "files" vector, adding 1 for spacing.
fn get_column_length(files: &Vec<File>) -> usize {
    files
        .iter()
        .max_by_key(|file| file.path_name.len())
        .unwrap()
        .path_name
        .len()
        + 1
}

// Returns the size of the file matrix in terms of the number of rows and columns.
fn get_matrix_size(
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
fn one_argument(target_path: &str) -> Result<Vec<File>, String> {
    let path = Path::new(target_path);

    match fs::read_dir(&path) {
        Ok(path) => Ok(create_files_vector(path)),
        Err(_) => Err(format!("ls: {}: No such file or directory", target_path).to_string()),
    }
}

fn main() {
    if env::args().len() == 1 {
        match one_argument("./") {
            Ok(files) => simple_print(files),
            Err(error_message) => println!("{}", error_message),
        }
    } else {
        handle_command();
    }
}
