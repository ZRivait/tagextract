use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use metaflac::Tag;
use regex::{Regex, Captures};
use crate::common;

pub struct Changes {

    path_to_file: PathBuf,
    fields: Vec<String>,
    values: Vec<String>,

}

impl Changes {

    fn new(path_to_file: PathBuf) -> Changes {

        Changes {
            path_to_file: path_to_file,
            fields: Vec::new(),
            values: Vec::new(),
        }

    }

    fn add_change(&mut self, field: &str, value: &str) {

        self.fields.push(String::from(field));
        self.values.push(String::from(value));

    }

    fn print_changes(&self) {

        let mut change_lines = String::new();

        let tag_struct = Tag::read_from_path(&self.path_to_file).unwrap();

        for (field, value) in self.fields.iter().zip(self.values.iter()) {

            let vorbis = tag_struct.vorbis_comments().unwrap();
            let old_value = match vorbis.get(&field) {

                Some(val) => val[0].as_str(),
                None => "none",

            };

            if old_value != value {
                change_lines.push_str(&format!("old: {}, new: {}\n", old_value, value));
            }

        }

        if !change_lines.is_empty() {

            println!("Changes for {:?}", self.path_to_file);
            print!("{}", change_lines);

        }

    }

    fn make_changes(&mut self) {

        let tag_struct = Tag::read_from_path(&self.path_to_file).unwrap();

        for (field, value) in self.fields.iter().zip(self.values.iter()) {

            let mut vorbis = tag_struct.vorbis_comments_mut().unwrap();
            let old_value = match vorbis.get(&field) {

                Some(val) => val[0].as_str(),
                None => "none",

            };

            if old_value != value {
                vorbis.set(field, vec![value]);
            }

        }

        tag_struct.save();

    }

}

// builds the input format specifier out of regex capture groups
// checks the captured tags and replaces them in the format specifier
// format: the format specifier
// captured_tags: the captured tags
// returns the new regex filed format specifier as a string
fn build_input_specifier(format: &str, captured_tags: &Vec<String>)  -> String {

    // adds the beginning and ending anchors
    let mut format_input = format!("^{}$", format);
    format_input = common::sanitize_for_regex(&format);

    // builds the input format specifier
    // replaces the tags in the input string with regex expressions
    for key in captured_tags {
    
        let re = Regex::new(format!(r"%(?P<tag>{})%", key).as_str()).unwrap();
        format_input = re.replace(&format_input, |cap: &Captures| {

            format!(r"(?P<{}>.+)", &cap["tag"])

        }).to_string();
    }

    format_input

}

// reads the tags from the text file and puts the line in a vector
// returns a vector of the line in the file
pub fn read_lines_from_file() -> Vec<String> {

    let file = OpenOptions::new()
        .read(true)
        .open("tags.txt")
        .unwrap();

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    // reads the lines in the tags file
    for line in reader.lines() {

        lines.push(line.unwrap());

    }           

    lines

}

pub fn create_changes(pwd: PathBuf, format: &str) -> Vec<Changes> {

    let captured_tags = common::get_format_tags(&format);
    let input_format = build_input_specifier(&format, &captured_tags);

    let lines = read_lines_from_file();
    let tags = common::get_flacs_sorted(pwd);

    let mut changes: Vec<Changes> = Vec::new();

    for tag in tags {

        let mut change = Changes::new(tag);
        changes.push(change);

    }

    for (line, mut change) in lines.iter().zip(changes.iter_mut()) {

        let re = Regex::new(&input_format).unwrap();

        let caps = re.captures(&line).unwrap();

        for key in &captured_tags {

            change.add_change(&key.to_uppercase(), caps.name(&key).unwrap().as_str());

        }

    }                        

    changes

}

pub fn print_changes(pwd: PathBuf, format: &str) {

    let changes = create_changes(pwd, format);

    for change in changes {

        change.print_changes();

    }

}

pub fn make_changes(pwd: PathBuf, format: &str) {

    let changes = create_changes(pwd, format);

    for change in changes {

        change.make_changes();

    }

}
