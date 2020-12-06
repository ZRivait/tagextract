use std::{fs, env};
use std::path::{Path, PathBuf};
use metaflac::Tag;
use regex::Regex;

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

    if format.is_empty() {

        panic!("no format specifier given");

    }

    let captured_tags = format_checker(&format);

    for tag in captured_tags.iter() {

        println!("{}", tag);

    }

    let pwd = match env::current_dir() {

        Ok(x) => x,
        Err(_) => panic!("could not get working directory"),

    };

    println!("pwd:{:?}", pwd);

    
    let flacs = get_flacs_sorted(pwd);
    let mut tags = Vec::new();

    for flac in flacs {

        let tag = match Tag::read_from_path(flac) {

            Ok(x) => x,
            Err(_) => panic!("error reading flac"),

        };

        tags.push(tag);

    }
    

    


}

// gets the flacs in the given directory
// passes them back as a sorted vec 
fn get_flacs_sorted(pwd: PathBuf) -> Vec<PathBuf> {

    let mut flacs: Vec<PathBuf> = Vec::new();

    let dir = match fs::read_dir(pwd) {
    
        Ok(x) => x,
        Err(_) => panic!("error opening directory")

    };

    for entry in dir {

        let file = match entry{

            Ok(x) => x,
            Err(_) => panic!("io error")

        };
        
        let path = file.path();

        let is_flac = match path.extension() {

            Some(x) => x == "flac",
            None => false

        };

        if is_flac {
            flacs.push(path);
        }
    }

    flacs.sort();

    flacs
}

// pulls the tags out of the format string
fn format_checker(format: &str) -> Vec<String> {

    let tag = Regex::new(r"%(?P<tag>[a-zA-Z]+)%").unwrap();
    let mut captured_tags = Vec::new();

    for caps in tag.captures_iter(format) {

        match &caps["tag"] {

            "artist" | "title" | "album" | "albumartist" | "track" | "disc" | "genre" | "year" | "comment" => captured_tags.push(String::from(&caps["tag"])),
            _ => (),

        }

    }

    captured_tags

}
