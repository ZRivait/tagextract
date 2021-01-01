use std::env;
use std::process;

mod common;
mod extract;
mod insert;

fn main() {

    let mut args = env::args();

    // skips the executable argument
    args.next();
    
    // gets the operation argument
    let operation = match args.next() {

        Some(operation) => operation, 
        None => {

            println!("No operation given. Exiting");
            process::exit(1);

        }

    };

    let mut format = String::new();

    let mut outfile = String::from("tags.txt");

    let mut unsupported_tags = false;

    // process the other arguments
    for arg in args {

        if arg.starts_with('-') {
            match arg.as_str() {

                "-o" | "--out" => {

                    match args.next() {

                        Some(x) => outfile = String::from(x),
                        None => {
                            println!("invalid argument");
                            process::exit(1);
                        }
                    }
                }
                "-i" | "--in" => (),
                "-u" | "--unsupported-tags" => (),
                _ => (),

            }
        }
        else {

            format = arg.clone();

        }
    }

    // panics if no format specifier is given
    if format.is_empty() {

        println!("No format specifier given. Exiting");
        process::exit(1);

    }

    // panics if given unsupported tags
    if !unsupported_tags {
        if !common::is_supported_tags(&format) {

            println!("Unsupported tags found in format specifier. Please pass the --unsupported-tags option. Exiting");
            process::exit(1);

        }
    }

    let pwd = match env::current_dir() {

        Ok(x) => x,
        Err(_) => {
            println!("Could not get pwd. Exiting");
            process::exit(1);
        }

    };

    let result = match operation.as_str() {

         "extract" => {
            extract::write_tags_to_file(pwd, &format)
         }

         "insert" => {
            insert::make_changes(pwd, &format)
         }

         "print" => {
            insert::print_changes(pwd, &format)
         }

         _ => {
            println!("Invalid operation. Exiting");
            process::exit(1);
         }

    };

    match result {

        Ok(_) => (), 
        Err(err) => {
            match err {
                common::TagError::IoError(_) => println!("Error: {}", err),
                common::TagError::MetaflacError(_) => println!("Error: {}", err),
            }
        }
    }
}
