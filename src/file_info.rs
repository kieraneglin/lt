// This file contains functions related to display of file metadata
use std::fs::Metadata;
use std::path::PathBuf;
use separator::Separatable;
use unicode_width::UnicodeWidthStr;

pub struct FileInfo {
    pub filepath: PathBuf,
    pub metadata: Metadata,
}

impl FileInfo {
    pub fn formatted_filepath(&self) -> String {
        let mut basename = self.filepath.display().to_string();
        if self.metadata.is_dir() {
            basename += "/";
        }

        basename
    }

    pub fn normalized_filename_length(&self) -> usize {
        UnicodeWidthStr::width(self.formatted_filepath().as_str())
    }

    pub fn filesize(&self) -> usize {
        self.metadata.len() as usize
    }

    pub fn formatted_filesize(&self) -> String {
        self.filesize().separated_string()
    }
}
