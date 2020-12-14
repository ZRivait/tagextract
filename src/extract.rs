use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use regex::Regex;
use crate::common;

// writes the tags to a file based on the given format specifier
// tags: the tags of the flac files to write
// format: the format specifier to base the output on
pub fn write_tags_to_file(pwd: PathBuf, format: &str) {

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tags.txt")
        .unwrap();

    let mut writer = BufWriter::new(file);

    let captured_tags = common::get_format_tags(&format);
    let tags = common::get_flac_tags(pwd);

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
