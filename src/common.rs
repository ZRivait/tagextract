use std::fs;
use std::path::PathBuf;
use metaflac::Tag;
use regex::Regex;

// gets the flacs in the given directory
// pwd: the path to check for flac files
// returns the paths back as a sorted vector
pub fn get_flacs_sorted(pwd: PathBuf) -> Vec<PathBuf> {

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
pub fn get_flac_tags(pwd: PathBuf) -> Vec<Tag> {

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
pub fn get_format_tags(format: &str) -> Vec<String> {

    let tag = Regex::new(r"%(?P<tag>[a-zA-Z]+)%").unwrap();
    let mut captured_tags = Vec::new();

    for caps in tag.captures_iter(format) {

        captured_tags.push(String::from(&caps["tag"])),

    }

    captured_tags

}
