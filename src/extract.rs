use std::{path::PathBuf, fs::OpenOptions};
use std::io::{BufWriter, Write};
use regex::Regex;
use crate::common;

// writes the tags to a file based on the given format specifier
// tags: the tags of the flac files to write
// format: the format specifier to base the output on
// outfile: the file to print the tags to 
pub fn write_tags_to_file(pwd: PathBuf, format: &str, outfile: &str) -> Result<(), common::TagError> {

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(outfile)?;

    let mut writer = BufWriter::new(file);

    let captured_tags = common::get_format_tags(&format);
    let tags = common::get_flac_tags(pwd);

    // gets the vorbis comment from each tag
    // reads the tags and then builds the output string based on the format specifier
    for tag in tags {

        let mut formatted_output = format.to_string();

        // get the vorbis comment block for a file
        // if one isnt found just create an empty blank one
        match tag.vorbis_comments() {

            // if a vorbis comment block is found grab the metadata from it
            // otherwise just replace all fields in the string with 'none'
            Some(vorbis) => { 
                for field in &captured_tags {

                    // get the values for the metadata field
                    // replaces the field in the format string with the value if found or 'none' otherwise
                    match vorbis.get(&field.to_uppercase()) {
                        
                        Some(tag_values) => formatted_output = replace_field_in_string(formatted_output, &field, tag_values[0].as_str()),
                        None => formatted_output = replace_field_in_string(formatted_output, &field, "none"),

                    }
                }
            }
            None => {
                for field in &captured_tags {

                     formatted_output = replace_field_in_string(formatted_output, &field, "none");

                }
            }
        }

        writer.write_all(formatted_output.as_bytes())?;
        writer.write(b"\n")?;

    }

    writer.flush()?;

    Ok(())

}

fn replace_field_in_string(string: String, field: &str, value: &str) -> String {

    // regex for the current field in the format specifier
    // this will always be valid regex, hence the unwrap
    let field_re = Regex::new(format!("%{}%", field).as_str()).unwrap();

    field_re.replace(&string, value).to_string()

}
