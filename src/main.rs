use std::env;

mod common;
mod extract;
mod insert;

fn main() {

    let mut args = env::args();

    // skips the executable argument
    args.next();
    
    // gets the operation argument
    let insert = match args.next() {

        Some(op) => 
            match op.as_str() {
            
                "i" => true,
                "x" => false,
                _ => panic!("incorrect operation"),
            }
        None => panic!("no operation given"),

    };

    let mut format = String::new();
    
    // process the other arguments
    for arg in args {

        if arg.starts_with('-') {
            match arg.as_str() {

                "-o" | "--out" => (),
                _ => (),

            }
        }
        else {

            format = arg.clone();

        }
    }

    // panics if no format specifier is given
    if format.is_empty() {

        panic!("no format specifier given");

    }

    // panics if given unsupported tags
    if !common::is_supported_tags(&format) {

        panic!("using unsupported tags");

    }

    let captured_tags = common::get_format_tags(&format);

    for tag in captured_tags.iter() {

        println!("{}", tag);

    }

    let pwd = match env::current_dir() {

        Ok(x) => x,
        Err(_) => panic!("could not get working directory"),

    };

    println!("pwd:{:?}", pwd);

    match insert {

        false => extract::write_tags_to_file(pwd, &format),
        true => insert::run_changes(pwd, &format),

    };

}



