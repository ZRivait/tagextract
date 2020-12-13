use std::{fs, env};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, BufWriter, Write};
use metaflac::Tag;
use regex::Regex;
use regex::Captures;

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

    let captured_tags = get_format_tags(&format);

    for tag in captured_tags.iter() {

        println!("{}", tag);

    }

    let pwd = match env::current_dir() {

        Ok(x) => x,
        Err(_) => panic!("could not get working directory"),

    };

    println!("pwd:{:?}", pwd);

    match insert {

        false => write_tags_to_file(pwd, &format),
        true => read_tags_from_file(&format),

    };

}

// gets the flacs in the given directory
// pwd: the path to check for flac files
// returns the paths back as a sorted vector
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

// gets the tags for each flac in a directory
// pwd: the path to check for flacs
// Returns a vector of the tags
fn get_flac_tags(pwd: PathBuf) -> Vec<Tag> {

    let flacs = get_flacs_sorted(pwd);
    let mut tags = Vec::new();

    // gets the Tag structs for each flac found
    for flac in flacs {

        let tag = match Tag::read_from_path(flac) {

            Ok(x) => x,
            Err(_) => panic!("error reading flac"),

        };

        tags.push(tag);

    }

    tags

}


// pulls the tags out of the format string
// format: the format string to get tags from
// returns the captured tags as a vector of strings
fn get_format_tags(format: &str) -> Vec<String> {

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

// writes the tags to a file based on the given format specifier
// tags: the tags of the flac files to write
// format: the format specifier to base the output on
fn write_tags_to_file(pwd: PathBuf, format: &str) {


    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tags.txt")
        .unwrap();

    let mut writer = BufWriter::new(file);

    let captured_tags = get_format_tags(&format);
    let tags = get_flac_tags(pwd);

    // gets the vorbis comment from each tag
    // reads the tags and then builds the output string based on the format specifier
    for tag in tags {

        let vorbis = tag.vorbis_comments().unwrap();
        let mut format_output = format.to_string();

        for key in &captured_tags {

            let val = vorbis.get(&key.to_uppercase()).unwrap();
            let re = Regex::new(format!("%{}%", key).as_str()).unwrap();
            format_output = re.replace(&format_output, val[0].as_str()).to_string();

        }

        writer.write_all(format_output.as_bytes());
        writer.write(b"\n");

    }

    writer.flush().unwrap();

}

// builds the input format specifier out of regex capture groups
// checks the captured tags and replaces them in the format specifier
// format: the format specifier
// returns the new regex filed format specifier as a string
fn build_input_specifier(format: &str)  -> String {

    let captured_tags = get_format_tags(&format);
    // adds the beginning and ending anchors
    let mut format_input = format!("^{}$", format);

    // builds the input format specifier
    // replaces the tags in the input string with regex expressions
    for key in &captured_tags {
    
        let re = Regex::new(format!("%{}%", key).as_str()).unwrap();
        format_input = re.replace(&format_input, |cap: &Captures| {

            match &cap[0] {

                "%artist%" => r"(?P<artist>.+)", 
                "%title%" => r"(?P<title>.+)", 
                "%album%" => r"(?P<album>.+)", 
                "%albumartist%" => r"(?P<albumartist>.+)", 
                "%track%" => r"(?P<track>.+)", 
                "%disc%" => r"(?P<disc>.+)", 
                "%genre%" => r"(?P<genre>.+)", 
                "%year%" => r"(?P<year>.+)",
                "%comment%" => r"(?P<comment>.+)",
                _ => "",

            }

        }).to_string();
    }

    format_input

}

// reads and seperates all the tags out of the file
// format: the format specifier
fn read_tags_from_file(format: &str) {

    let file = OpenOptions::new()
        .read(true)
        .open("tags.txt")
        .unwrap();

    let reader = BufReader::new(file);

    let captured_tags = get_format_tags(&format);
    let input_format = build_input_specifier(&format);

    // applies the built input format specifier to read the lines in the tags file
    for line in reader.lines() {

        let text = line.unwrap();
        println!("{}", text);
        let re = Regex::new(&input_format).unwrap();

        let caps = re.captures(&text).unwrap();

        for key in &captured_tags {

            println!("{} : {}", key, caps.name(&key).unwrap().as_str());

        }

    }                            
}

