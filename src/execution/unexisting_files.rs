use crate::*;

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

fn remove_unexisting_files(args: &mut Vec<String>, unexisting_files: &mut Vec<String>) {
    let mut counter = 0;

    while counter != unexisting_files.len() {
        let index = return_index_for_object(args, &unexisting_files[counter]);
        args.remove(index);
        counter += 1;
    }
}

fn simple_print_unexisting_files(unexisting_files: &Vec<String>) {
    for i in unexisting_files {
        println!("ls: {}: No such file or directory", i);
    }
}

pub fn handle_unexisting_files(args: &mut Vec<String>) {
    let mut unexisting_files = find_unexisting_files(args);

    remove_unexisting_files(args, &mut unexisting_files);
    alphabetically_rank_strings(&mut unexisting_files);

    simple_print_unexisting_files(&unexisting_files);
}
