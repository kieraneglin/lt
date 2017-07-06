extern crate separator;

use std::fs::Metadata;
use std::path::PathBuf;
use separator::Separatable;

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

    pub fn filesize(&self) -> usize {
        self.metadata.len() as usize
    }

    pub fn formatted_filesize(&self) -> String {
        self.metadata.len().separated_string()
    }
}
