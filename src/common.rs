use std::{error::Error, convert::From, fmt, fs, io};
use std::path::PathBuf;
use metaflac::{Tag};
use regex::Regex;

#[derive(Debug)]
pub enum TagError {

    IoError(String),
    MetaflacError(String),

}

impl Error for TagError {}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {

            TagError::IoError(desc) => write!(f, "{}", desc),
            TagError::MetaflacError(desc) => write!(f, "{}", desc),

        }
    }
}

impl From<io::Error> for TagError {

    fn from(err: io::Error) -> TagError {

        TagError::IoError(String::from("an io error"))

    }

}

impl From<metaflac::Error> for TagError {

    fn from(err: metaflac::Error) -> TagError {

        TagError::MetaflacError(String::from(err.description))

    }

}

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

    let tag_re = Regex::new(r"%(?P<tag>[a-zA-Z]+)%").unwrap();
    let mut captured_tags = Vec::new();

    for caps in tag_re.captures_iter(format) {

        captured_tags.push(String::from(&caps["tag"]));

    }

    captured_tags

}

// checks if the tags in the format string are in the supported list
// format: the format string with the tags
// returns a bool if its false or not
pub fn is_supported_tags(format: &str) -> bool {

    let tag_re = Regex::new(r"%(?P<tag>[a-zA-Z]+)%").unwrap();

    for caps in tag_re.captures_iter(format) {

        match &caps["tag"] {

                "artist" | "title" | "album" | "albumartist" | "tracknumber" | "discnumber" | "genre" | "date" | "comment" => (),
                _ => return false,

        }

    }

    true

}

// sanitizes a string for use in regex expressions by escaping metacharacters
// string: the string to be sanitized
// Returns a copy of the string post-sanitization
pub fn sanitize_for_regex(string: &str) -> String {

    let meta_re = Regex::new(r"(?P<meta>[\\\.\+\*\?\(\)\|\{\}\[\]\^\$])").unwrap();

    meta_re.replace_all(string, r"\$meta").to_string()

}
