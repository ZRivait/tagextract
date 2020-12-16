use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Write};
use metaflac::block::VorbisComment;
use regex::Regex;
use crate::common;

// writes the tags to a file based on the given format specifier
// tags: the tags of the flac files to write
// format: the format specifier to base the output on
pub fn write_tags_to_file(pwd: PathBuf, format: &str) -> Result<(), io::Error> {

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tags.txt")?;

    let mut writer = BufWriter::new(file);

    let captured_tags = common::get_format_tags(&format);
    let tags = common::get_flac_tags(pwd);

    // gets the vorbis comment from each tag
    // reads the tags and then builds the output string based on the format specifier
    for tag in tags {

        // get the vorbis comment block for a file
        // if one isnt found just create an empty blank one
        let vorbis = match tag.vorbis_comments() {

            Some(comments) => comments,
            None => &VorbisComment::new(),

        };
        let mut formatted_output = format.to_string();

        for field in &captured_tags {

            // get the values for the metadata field
            let tag_values = match vorbis.get(&field.to_uppercase()) {
                
                Some(values) => values,
                None => &vec![String::from("none")],

            };

            // regex for the current field in the format specifier
            // this will always be valid regex, hence the unwrap
            let field_re = Regex::new(format!("%{}%", field).as_str()).unwrap();

            // replace the field in the format specifier with the found value
            formatted_output = field_re.replace(&formatted_output, tag_values[0].as_str()).to_string();

        }

        writer.write_all(formatted_output.as_bytes());
        writer.write(b"\n");

    }

    writer.flush()?;

    Ok(())

}
