use std::cmp::max;
use file_info::FileInfo;
use separator::Separatable;

const HEADER_LENGTH: usize = 8; // 8 happens to be the size of both "Filename" and "Filesize"
const PADDING_OFFSET: usize = 2; // 2, since there's 1 space on each side of a table element

pub struct Table {
    pub filename: usize, // Note: These aren't for file metadata
    pub filesize: usize, // filename and filesize reflect the literal display name of the columns
}

impl Table {
    pub fn attribute_without_padding(attr: usize) -> usize {
        // The table width will be determined here.  If the longest element is
        // shorter than the title, go off the title's width.
        max(HEADER_LENGTH, attr) // 8 happens to be the size of both "Filename" and "Filesize"
    }

    pub fn inner_computed_table(filename_width: usize, filesize_width: usize) -> Self {
        // I know hardcoded string lengths are the devil, but they'll always be the same lenght.
        // The numbers correspond to the lengths of "filename" and "filesize" with padding spaces
        let min_filename_width = HEADER_LENGTH;
        let min_filesize_width = HEADER_LENGTH;

        // Add 2 for the padding spaces.
        let actual_filename_width = max(
            min_filename_width + PADDING_OFFSET,
            filename_width + PADDING_OFFSET,
        );
        let actual_filesize_width = max(
            min_filesize_width + PADDING_OFFSET,
            filesize_width + PADDING_OFFSET,
        );

        Self {
            filename: actual_filename_width,
            filesize: actual_filesize_width,
        }
    }

    // We need to find the max length of the filesize and filepath,
    // so that we know how wide to make the table
    pub fn numeric_table(fileinfo: &Vec<FileInfo>) -> usize {
        let mut result: usize = 0;

        for file in fileinfo {
            let width = file.filesize() as usize;

            if width > result {
                result = width;
            }
        }

        result.separated_string().to_string().len()
    }

    pub fn string_table(fileinfo: &Vec<FileInfo>) -> usize {
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
