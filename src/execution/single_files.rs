use crate::*;

fn find_single_files(args: &mut Vec<String>) -> Vec<String> {
    let mut counter = 0;
    let mut single_files: Vec<String> = Vec::new();

    while counter != args.len() {
        if is_file(&args[counter]) {
            single_files.push(args[counter].to_string());
        }
        counter += 1;
    }

    single_files
}

fn remove_single_files(args: &mut Vec<String>, single_files: &mut Vec<String>) {
    let mut counter = 0;

    while counter != single_files.len() {
        let index = return_index_for_object(args, &single_files[counter]);
        args.remove(index);
        counter += 1;
    }
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

pub fn handle_single_files(args: &mut Vec<String>, parameters: &Parameters) -> bool {
    let mut single_files = find_single_files(args);

    remove_single_files(args, &mut single_files);

    if parameters.reverse_order == true {
        reverse_alphabetically_rank_strings(&mut single_files);
    } else {
        alphabetically_rank_strings(&mut single_files);
    }

    if !single_files.is_empty() {
        simple_print_single_files(&single_files);
        return false;
    }
    return true;
}
