#[macro_use]
extern crate clap;

extern crate glob;
extern crate separator;

mod table;
mod file_info;
mod table_format;

use glob::glob_with;
use std::path::Path;
use glob::MatchOptions;
use file_info::FileInfo;
use table_format::TableFormat;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file_pattern: String = matches.value_of("pattern").unwrap_or("*").to_string();
    let show_dotfiles = matches.is_present("dotfiles");
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        // We need this to return false if the flag is present.  Hence the !
        require_literal_leading_dot: !show_dotfiles,
    };
    let mut fileinfo: Vec<FileInfo> = vec![];

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

    TableFormat::print_table(&fileinfo);
}
