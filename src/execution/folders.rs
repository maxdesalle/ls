use crate::*;
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::fs::read_link;
use std::fs::{read_dir, ReadDir};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use xattr;

pub fn check_extended_attributes(path: &Path) -> bool {
    match xattr::list(path) {
        Ok(attributes) => {
            if attributes.peekable().peek().is_none() {
                return false;
            } else {
                return true;
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            return false;
        }
    }
}

fn insert_dot_files_in_vector(files: &mut Vec<File>) {
    let current_folder = Path::new(".");
    let parent_folder = Path::new("..");

    let current_metadata = current_folder.metadata().unwrap();
    let parent_metadata = parent_folder.metadata().unwrap();

    files.push(File::new(
        ".".to_string(),
        current_metadata,
        check_extended_attributes(current_folder),
    ));
    files.push(File::new(
        "..".to_string(),
        parent_metadata,
        check_extended_attributes(parent_folder),
    ));
}

// Assembles the vector returned in create_files_vector() by filling each File object with the
// given metadata.
fn insert_path_in_vector(paths: ReadDir, files: &mut Vec<File>, parameters: &Parameters) {
    if parameters.include_dot_files == true {
        insert_dot_files_in_vector(files);
    }

    for path in paths {
        match path {
            Ok(path) => {
                let metadata = path.metadata().unwrap();
                files.push(File::new(
                    get_path_name(&path.path()),
                    metadata,
                    check_extended_attributes(&path.path()),
                ));
            }
            Err(error_message) => println!("{}", error_message),
        }
    }
}

// Returns a simple vector of File objects based on the given path.
fn create_files_vector(paths: ReadDir, parameters: &Parameters) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    insert_path_in_vector(paths, &mut files, parameters);

    if parameters.reverse_order == true {
        if parameters.last_modified_order == true {
            reverse_rank_files_by_last_modified_date(&mut files);
        } else {
            reverse_alphabetically_rank_files(&mut files);
        }
    } else {
        if parameters.last_modified_order == true {
            rank_files_by_last_modified_date(&mut files);
        } else {
            alphabetically_rank_files(&mut files);
        }
    }

    return files;
}

fn handle_single_arguments(target_path: &str, parameters: &Parameters) {
    if is_file(target_path) {
        println!("{}", target_path);
    } else {
        match one_argument(target_path, parameters) {
            Ok(files) => {
                if parameters.long_format == true {
                    long_format_print(files, parameters, false)
                } else {
                    simple_print(files, parameters)
                }
            }
            Err(error_message) => println!("{}", error_message),
        }
    }
}

fn file_type(file: &File) -> String {
    if file.is_dir == true {
        String::from("d")
    } else if file.is_symbolic_link == true {
        String::from("l")
    } else {
        String::from("-")
    }
}

fn permission_bits(mode: u32, read: u32, write: u32, execute: u32) -> String {
    let r = if mode & read != 0 { "r" } else { "-" };
    let w = if mode & write != 0 { "w" } else { "-" };
    let x = if mode & execute != 0 { "x" } else { "-" };

    format!("{}{}{}", r, w, x)
}

fn get_longest_number_of_links(files: &Vec<File>) -> usize {
    let mut longest_number = 1;

    for i in files {
        if i.number_of_links.to_string().len() > longest_number {
            longest_number = i.number_of_links.to_string().len();
        }
    }
    longest_number
}

fn get_longest_file_size(files: &Vec<File>) -> usize {
    let mut longest_file_size = 1;

    for i in files {
        if i.number_of_bytes.to_string().len() > longest_file_size {
            longest_file_size = i.number_of_bytes.to_string().len();
        }
    }
    longest_file_size
}

fn get_total_number_of_blocks(files: &Vec<File>) -> u64 {
    let mut total_number_of_blocks = 0;

    for i in files {
        total_number_of_blocks += i.blocks;
    }
    total_number_of_blocks
}

fn print_permissions(file: &File) {
    let mode = file.file_mode.mode();

    // Print the file permissions in the format of the "ls -l" command
    print!("{}", file_type(&file));
    print!("{}", permission_bits(mode, 0o400, 0o200, 0o100));
    print!("{}", permission_bits(mode, 0o040, 0o020, 0o010));
    print!("{}", permission_bits(mode, 0o004, 0o002, 0o001));

    if file.extended_attributes == true {
        print!("@ ");
    } else {
        print!("  ");
    }
}

fn print_spacing_difference(number_one: usize, number_two: usize) {
    let mut spacing = number_one - number_two;

    while spacing > 0 {
        print!(" ");
        spacing -= 1;
    }
}

fn get_symbolic_link(file: &File) -> String {
    read_link(&file.path_name)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn print_file_name_long_format(file: &File) {
    print!("{}", color_print(&file));
    if file.is_symbolic_link == true {
        print!(" -> {}", get_symbolic_link(file));
    }
    println!();
}

fn print_date_long_format(file: &File) {
    let datetime: DateTime<Local> = file.last_modified.into();
    let formatted = datetime.format("%b %e %H:%M").to_string();
    print!("{} ", formatted);
}

pub fn long_format_print(mut files: Vec<File>, parameters: &Parameters, single_files: bool) {
    // Remove all the files where the name starts with a dot, if the -a parameter was not included.
    if parameters.include_dot_files == false {
        files.retain(|file| !file.path_name.starts_with('.'));
    }

    let longest_number = get_longest_number_of_links(&files);
    let longest_file_size = get_longest_file_size(&files);

    if single_files == false {
        println!("total {}", get_total_number_of_blocks(&files));
    }

    for file in files {
        print_permissions(&file);
        print_spacing_difference(longest_number, file.number_of_links.to_string().len());

        print!("{} ", file.number_of_links);
        print!("{}  ", file.owner_name);
        print!("{}  ", file.group_name);

        print_spacing_difference(longest_file_size, file.number_of_bytes.to_string().len());
        print!("{} ", file.number_of_bytes);
        print_date_long_format(&file);
        print_file_name_long_format(&file);
    }
}

// Handle commands with multiple files or directories to list, but without any parameter.
fn handle_folders(args: Vec<String>, multiple_arguments: bool, parameters: &Parameters) {
    let counter = 0;

    // if args[0].chars().nth(0).unwrap() != '-' {
    if args.len() > 1 || multiple_arguments == true {
        handle_multiple_arguments(args, parameters);
    } else {
        handle_single_arguments(&args[counter], parameters);
    }
    // }
}

fn print_format_redirect(files: Vec<File>, parameters: &Parameters) {
    if parameters.long_format == true {
        long_format_print(files, parameters, false);
    } else {
        simple_print(files, parameters);
    }
}

// Iterates through all the command's arguments to print them one by one according to the
// formatting of the "ls" command.
fn handle_multiple_arguments(args: Vec<String>, parameters: &Parameters) {
    let mut counter = 0;

    while counter != args.len() {
        if is_file(&args[counter]) {
            println!("{}", &args[counter]);
            if counter != args.len() - 1 {
                println!();
            }
        } else {
            match one_argument(&args[counter], parameters) {
                Ok(files) => {
                    print!("{}:", &args[counter]);
                    if !files.is_empty() {
                        println!();
                    }
                    print_format_redirect(files, parameters);
                    if counter != args.len() - 1 {
                        println!();
                    }
                }
                Err(error_message) => println!("{}", error_message),
            }
        }
        counter += 1;
    }
}

fn parse_parameters(args: &mut Vec<String>) -> Parameters {
    let mut parameters = Parameters::new();

    if !args[0].starts_with("-") {
        return parameters;
    }

    if args[0].contains("a") {
        parameters.include_dot_files = true;
    }

    if args[0].contains("l") {
        parameters.long_format = true;
    }

    if args[0].contains("r") {
        parameters.reverse_order = true;
    }

    if args[0].contains("t") {
        parameters.last_modified_order = true;
    }

    if args[0].contains("R") {
        parameters.recursive_listing = true;
    }

    args.remove(0);
    if args.is_empty() {
        args.push("./".to_string());
    }

    return parameters;
}

fn check_parameters(parameters: &Parameters, mut args: Vec<String>) -> Vec<String> {
    if parameters.reverse_order == true {
        if parameters.last_modified_order == true {
            reverse_rank_path_by_last_modified_date(&mut args);
        } else {
            reverse_alphabetically_rank_strings(&mut args);
        }
    } else if parameters.last_modified_order == true {
        rank_path_by_last_modified_date(&mut args);
    }

    args
}

fn directory_traversal(path: &PathBuf, parameters: &Parameters) -> Vec<PathBuf> {
    let mut directories: Vec<PathBuf> = Vec::new();
    // let directory = read_dir(path).unwrap();
    let mut directory: Vec<PathBuf> = read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    if parameters.reverse_order == true {
        if parameters.last_modified_order == true {
            reverse_rank_path_bufs_by_last_modified_date(&mut directory);
        } else {
            reverse_alphabetically_rank_path_bufs(&mut directory);
        }
    } else if parameters.last_modified_order == true {
        rank_path_bufs_by_last_modified_date(&mut directory);
    } else {
        alphabetically_rank_path_bufs(&mut directory);
    }

    for entry in directory {
        if entry.is_dir() && !(get_path_name(&entry).starts_with('.')) {
            // println!("{}", entry.file_name().unwrap().to_str().unwrap());
            directories.push(entry.clone());
            directories.append(&mut directory_traversal(&entry, &parameters));
        }
    }
    return directories;
}

fn handle_recursivity(args: &mut Vec<String>, parameters: &Parameters) {
    let mut directories: Vec<PathBuf> = Vec::new();
    let mut single_files: Vec<File> = Vec::new();
    let number_of_arguments = args.len();

    alphabetically_rank_strings(args);

    if handle_single_files(args, parameters) == false && !args.is_empty() {
        println!();
    }

    for i in &mut *args {
        let path = Path::new(&i);
        if path.is_dir() {
            if number_of_arguments > 1 {
                directories.push(path.to_path_buf());
            }
            directories.append(&mut directory_traversal(&path.to_path_buf(), &parameters));
        } else {
            single_files.push(File::new(
                get_path_name(&path.to_path_buf()),
                path.metadata().unwrap(),
                check_extended_attributes(&path),
            ));
        }
    }

    // println!("{:?}", directories);

    if args.len() == 1 {
        match one_argument(&args[0], &parameters) {
            Ok(mut files) => {
                files.append(&mut single_files);
                if parameters.long_format == true {
                    long_format_print(files, parameters, false);
                } else {
                    simple_print(files, &parameters);
                }
            }
            Err(error_message) => println!("{}", error_message),
        }
        if !directories.is_empty() {
            println!();
        }
    }

    handle_folders(
        convert_path_buf_vector_to_string_vector(&directories),
        true,
        parameters,
    );
}

// Handles both commands with multiple arguments but without parameters, and vice-versa.
pub fn handle_command(mut args: Vec<String>) {
    // let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    let parameters = parse_parameters(&mut args);
    handle_unexisting_files(&mut args);

    if parameters.recursive_listing == true {
        return handle_recursivity(&mut args, &parameters);
    }

    let empty_single_files = handle_single_files(&mut args, &parameters);

    args = check_parameters(&parameters, args);

    if !args.is_empty() {
        if empty_single_files == true {
            handle_folders(args, false, &parameters);
        } else {
            println!();
            handle_folders(args, true, &parameters);
        }
    }
}

// Directories and files are colored differently, which is why this function is needed.
fn color_print(file: &File) -> ColoredString {
    if file.is_dir == true {
        return format!("{}", file.path_name).cyan().bold();
    } else if file.is_symbolic_link == true {
        return format!("{}", file.path_name).purple();
    } else if is_executable(file) == true {
        return format!("{}", file.path_name).red();
    } else {
        return format!("{}", file.path_name).white();
    }
}

// The function used when all that is needed is to output the files, without information about
// them.
pub fn simple_print(mut files: Vec<File>, parameters: &Parameters) {
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

    // Remove all the files where the name starts with a dot, if the -a parameter was not included.
    if parameters.include_dot_files == false {
        files.retain(|file| !file.path_name.starts_with('.'));
    }
    if files.is_empty() {
        println!();
        return;
    }
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
pub fn one_argument(target_path: &str, parameters: &Parameters) -> Result<Vec<File>, String> {
    let path = Path::new(target_path);

    match read_dir(&path) {
        Ok(path) => Ok(create_files_vector(path, parameters)),
        Err(_) => Err(format!("ls: {}: No such file or directory", target_path).to_string()),
    }
}
