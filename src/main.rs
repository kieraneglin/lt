extern crate glob;
extern crate separator;

mod table;
mod file_info;
mod table_format;

use std::env;
use glob::glob_with;
use glob::MatchOptions;
use std::path::Path;
use file_info::FileInfo;
use table_format::TableFormat;

fn main() {
    let mut fileinfo: Vec<FileInfo> = vec![];
    let mut file_pattern: String = String::from("*");
    let mut args = env::args();

    if args.len() > 1 {
        file_pattern = args.nth(1).expect("Error parsing glob input")
    };

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with(&file_pattern, &options).expect("Failed to read directory") {
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

    TableFormat::print_table(fileinfo);
}
