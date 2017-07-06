use std::cmp::max;
use file_info::FileInfo;
use separator::Separatable;

pub struct TableWidth {
    pub filename: usize,
    pub filesize: usize,
}
//
impl TableWidth {
    pub fn attribute_without_padding(attr: usize) -> usize {
        // 8 happens to be the size of both "Filename" and "Filesize"
        max(8, attr)
    }

    pub fn inner_computed_table_width(filename_width: usize, filesize_width: usize) -> Self {
        // I know hardcoded string lengths are the devil, but they'll always be the same lenght.
        // The numbers correspond to the lengths of "filename" and "filesize" with padding spaces
        let min_filename_width = 10;
        let min_filesize_width = 10;

        // Add 2 for the padding spaces
        let actual_filename_width = max(min_filename_width, filename_width + 2);
        let actual_filesize_width = max(min_filesize_width, filesize_width + 2);

        Self {
            filename: actual_filename_width,
            filesize: actual_filesize_width,
        }
    }

    // We need to find the max length of the filesize and filepath,
    // so that we know how wide to make the table
    pub fn numeric_table_width(fileinfo: &Vec<FileInfo>) -> usize {
        let mut result: usize = 0;

        for file in fileinfo {
            let width = file.filesize() as usize;

            if width > result {
                result = width;
            }
        }

        result.separated_string().to_string().len()
    }

    pub fn string_table_width(fileinfo: &Vec<FileInfo>) -> usize {
        let mut result: usize = 0;

        for file in fileinfo {
            let width = file.formatted_filepath().len();

            if width > result {
                result = width;
            }
        }

        result
    }
}
