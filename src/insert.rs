use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use regex::{Regex, Captures};
use crate::common;

// builds the input format specifier out of regex capture groups
// checks the captured tags and replaces them in the format specifier
// format: the format specifier
// returns the new regex filed format specifier as a string
fn build_input_specifier(format: &str)  -> String {

    let captured_tags = common::get_format_tags(&format);
    // adds the beginning and ending anchors
    let mut format_input = format!("^{}$", format);
    format_input = common::sanitize_for_regex(&format);

    // builds the input format specifier
    // replaces the tags in the input string with regex expressions
    for key in &captured_tags {
    
        let re = Regex::new(format!(r"%(?P<tag>{})%", key).as_str()).unwrap();
        format_input = re.replace(&format_input, |cap: &Captures| {

            format!(r"(?P<{}>.+)", &cap["tag"])

        }).to_string();
    }

    format_input

}

// reads and seperates all the tags out of the file
// format: the format specifier
pub fn read_tags_from_file(format: &str) {

    let file = OpenOptions::new()
        .read(true)
        .open("tags.txt")
        .unwrap();

    let reader = BufReader::new(file);

    let captured_tags = common::get_format_tags(&format);
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
