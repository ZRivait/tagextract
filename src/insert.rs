use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use metaflac::{Tag};
use regex::{Regex, Captures};
use crate::common;

// holds a series of metadata changes to be made to a music file
// currently only support flac files but support may be expanded later
// each change is held in synchronized vectors as a pair of strings
pub struct Changes {

    path_to_file: PathBuf,
    fields: Vec<String>,
    values: Vec<String>,

}

impl Changes {

    // creates a new Changes struct
    // path_to_file: the path to the file the changes are meant for
    // returns the new changes file
    fn new(path_to_file: PathBuf) -> Changes {

        Changes {
            path_to_file: path_to_file,
            fields: Vec::new(),
            values: Vec::new(),
        }

    }

    // adds a new change to the struct
    // field: the metadata field to be changed
    // value: the new value of the field
    fn add_change(&mut self, field: &str, value: &str) {

        self.fields.push(String::from(field));
        self.values.push(String::from(value));

    }

    // prints the changes in a human readable way
    // only prints changes that actually change something
    fn print_changes(&self) -> Result<(), common::TagError> {

        let mut change_lines = String::new();

        let tag_struct = Tag::read_from_path(&self.path_to_file)?;

        for (field, value) in self.fields.iter().zip(self.values.iter()) {

            let vorbis = tag_struct.vorbis_comments().unwrap();
            let old_value = match vorbis.get(&field) {

                Some(val) => val[0].as_str(),
                None => "none",

            };

            if old_value != value {
                if value.as_str() != "none" {
                    change_lines.push_str(&format!("field: {}, old: {}, new: {}\n", field, old_value, value));
                }
                else {
                    change_lines.push_str(&format!("removing field: {}, old value: {}\n", field, old_value));
                }
            }

        }

        if !change_lines.is_empty() {

            println!("Changes for {:?}", self.path_to_file.file_name().unwrap());
            print!("{}", change_lines);

        }
        
        Ok(())

    }

    // makes the changes to the metadata the saves them
    fn make_changes(&mut self) -> Result<(), common::TagError> {

        let mut tag_struct = Tag::read_from_path(&self.path_to_file)?;

        for (field, value) in self.fields.iter().zip(self.values.iter()) {

            let vorbis = tag_struct.vorbis_comments_mut();
            let old_value = match vorbis.get(&field) {

                Some(val) => val[0].as_str(),
                None => "none",

            };

            if old_value != value {
                if value.as_str() != "none" {
                    vorbis.set(field, vec![value]);
                }
                else {
                    vorbis.remove(field);
                }
            }

        }

        tag_struct.save()?;

        Ok(())

    }

}

// builds the input format specifier out of regex capture groups
// checks the captured tags and replaces them in the format specifier
// format: the format specifier
// captured_tags: the captured tags
// returns the new regex filed format specifier as a string
fn build_input_specifier(format: &str, captured_tags: &Vec<String>)  -> String {

    // adds the beginning and ending anchors
    let mut input_format = format!("^{}$", format);
    input_format = common::sanitize_for_regex(&format);

    // builds the input format specifier
    // replaces the tags in the input string with regex expressions
    for key in captured_tags {
    
        let re = Regex::new(format!(r"%(?P<tag>{})%", key).as_str()).unwrap();
        input_format = re.replace(&input_format, |cap: &Captures| {

            format!(r"(?P<{}>.+)", &cap["tag"])

        }).to_string();
    }

    input_format

}

// reads the tags from the text file and puts the line in a vector
// outfile: the file to read the lines from
// returns a vector of the line in the file
pub fn read_lines_from_file(outfile: &str) -> Vec<String> {

    let file = OpenOptions::new()
        .read(true)
        .open(outfile)
        .unwrap();

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    // reads the lines in the tags file
    for line in reader.lines() {

        lines.push(line.unwrap());

    }           

    lines

}

// creates a list of Changes that are to be made
// pwd: the path to the directory the files to be changed are in
// format: the format specifier the changes are based on
// outfile: the file to read the changes from
// returns a vector of the new Changes
pub fn create_changes(pwd: PathBuf, format: &str, outfile: &str) -> Vec<Changes> {

    let captured_tags = common::get_format_tags(&format);
    let input_format = build_input_specifier(&format, &captured_tags);

    let lines = read_lines_from_file(outfile);
    let tags = common::get_flacs_sorted(pwd);

    let mut changes: Vec<Changes> = Vec::new();

    for tag in tags {

        let change = Changes::new(tag);
        changes.push(change);

    }

    for (line, change) in lines.iter().zip(changes.iter_mut()) {

        let re = Regex::new(&input_format).unwrap();

        let caps = re.captures(&line).unwrap();

        for key in &captured_tags {

            change.add_change(&key.to_uppercase(), caps.name(&key).unwrap().as_str());

        }

    }                        

    changes

}

// creates then prints the changes
// pwd: the path to the directory the files to be changed are in
// format: the format specifier the changes are based on
// outfile: the file to read the changes from
pub fn print_changes(pwd: PathBuf, format: &str, outfile: &str) -> Result<(), common::TagError> {

    let changes = create_changes(pwd, format, outfile);

    for change in changes {

        change.print_changes()?;

    }

    Ok(())

}

// creates then makes the changes
// pwd: the path to the directory the files to be changed are in
// format: the format specifier the changes are based on
// outfile: the file to read the changes from
pub fn make_changes(pwd: PathBuf, format: &str, outfile: &str) -> Result<(), common::TagError> {

    let changes = create_changes(pwd, format, outfile);

    for mut change in changes {

        change.make_changes()?;

    }

    Ok(())

}
