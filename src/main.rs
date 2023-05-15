use std::env;

mod utilities;
use utilities::helpers::*;
use utilities::structs::*;

mod execution;
use execution::folders::*;
use execution::single_files::*;
use execution::unexisting_files::*;

#[allow(warnings)]

fn main() {
    if env::args().len() == 1 {
        let parameters = Parameters::new();
        match one_argument("./", &parameters) {
            Ok(files) => simple_print(files, &parameters),
            Err(error_message) => println!("{}", error_message),
        }
    } else {
        handle_command(env::args().collect());
    }
}
