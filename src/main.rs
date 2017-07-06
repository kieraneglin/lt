extern crate glob;
extern crate separator;

use glob::glob;
use std::path::{Path, PathBuf};
use std::fs::Metadata;
use std::cmp::max;
use separator::Separatable;

struct TableWidth {
    filename: usize,
    filesize: usize,
}
//
impl TableWidth {
    fn attribute_without_padding(attr: usize) -> usize {
        // 8 happens to be the size of both "Filename" and "Filesize"
        max(8, attr)
    }
}

struct FileInfo {
    filepath: PathBuf,
    metadata: Metadata,
}

fn main() {
    let mut fileinfo: Vec<(PathBuf, Metadata)> = vec![];

    for entry in glob("*").expect("Failed to read directory") {
        let filepath = match entry {
            Ok(path) => path,
            Err(e) => panic!("Couldn't parse file. {}", e),
        };
        let metadata = match Path::new(&filepath).metadata() {
            Ok(data) => data,
            Err(e) => panic!("Couldn't parse metadata for {}. {}", filepath.display(), e),
        };

        fileinfo.push((filepath, metadata));
    }

    let filename_width: usize = string_table_width(&fileinfo);
    let filesize_width: usize = numeric_table_width(&fileinfo);
    let inner_width = inner_computed_table_width(filename_width, filesize_width);

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
            file.0.display().to_string(),
            file.1.len().separated_string(),
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

fn inner_computed_table_width(filename_width: usize, filesize_width: usize) -> TableWidth {
    // I know hardcoded string lengths are the devil, but they'll always be the same lenght.
    // The numbers correspond to the lengths of "filename" and "filesize" with padding spaces
    let min_filename_width = 10;
    let min_filesize_width = 10;

    // Add 2 for the padding spaces
    let actual_filename_width = max(min_filename_width, filename_width + 2);
    let actual_filesize_width = max(min_filesize_width, filesize_width + 2);

    println!("{}", actual_filename_width);
    println!("{}", actual_filesize_width);

    TableWidth {
        filename: actual_filename_width,
        filesize: actual_filesize_width,
    }
}

// We need to find the max length of the filesize and filepath,
// so that we know how wide to make the table
fn numeric_table_width(fileinfo: &Vec<(PathBuf, Metadata)>) -> usize {
    let mut result: usize = 0;

    for file in fileinfo {
        let width = file.1.len() as usize;

        if width > result {
            result = width;
        }
    }

    result.separated_string().to_string().len()
}

fn string_table_width(fileinfo: &Vec<(PathBuf, Metadata)>) -> usize {
    let mut result: usize = 0;

    for file in fileinfo {
        let width = file.0.display().to_string().len();

        if width > result {
            result = width;
        }
    }

    result
}
