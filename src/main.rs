extern crate glob;
extern crate separator;

mod file_info;
mod table_width;

use glob::glob;
use std::path::Path;
use std::cmp::max;
use file_info::FileInfo;
use table_width::*;

fn main() {
    let mut fileinfo: Vec<FileInfo> = vec![];

    for entry in glob("*").expect("Failed to read directory") {
        let filepath = match entry {
            Ok(path) => path,
            Err(e) => panic!("Couldn't parse file. {}", e),
        };
        let metadata = match Path::new(&filepath).metadata() {
            Ok(data) => data,
            Err(e) => panic!("Couldn't parse metadata for {}. {}", filepath.display(), e),
        };

        fileinfo.push(FileInfo { filepath, metadata });
    }

    let filename_width: usize = TableWidth::string_table_width(&fileinfo);
    let filesize_width: usize = TableWidth::numeric_table_width(&fileinfo);
    let inner_width = TableWidth::inner_computed_table_width(filename_width, filesize_width);

    println!(
        "┌{}┬{}┐",
        "─".repeat(inner_width.filename),
        "─".repeat(inner_width.filesize)
    );
    println!(
        "│ {:name$} │ {:size$} │",
        "Filename",
        "Filesize",
        name = filename_width,
        size = filesize_width
    );
    println!(
        "├{}┼{}┤",
        "─".repeat(inner_width.filename),
        "─".repeat(inner_width.filesize)
    );
    for file in fileinfo {
        println!(
            "│ {:name$} │ {:>size$} │",
            file.formatted_filepath(),
            file.formatted_filesize(),
            name = TableWidth::attribute_without_padding(filename_width),
            size = TableWidth::attribute_without_padding(filesize_width)
        );
    }
    println!(
        "└{}┴{}┘",
        "─".repeat(inner_width.filename),
        "─".repeat(inner_width.filesize)
    );
}
