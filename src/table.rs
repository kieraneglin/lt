// This file contains functions for calculating table offsets and widths.

use std::cmp::max;
use file_info::FileInfo;
use table_format::TableFormat;
use separator::Separatable;

const HEADER_WIDTH: usize = 8; // 8 happens to be the size of both "Filename" and "Filesize"
const PADDING_OFFSET: usize = 2; // 2, since there's 1 space on each side of a table element

pub struct Table {
    pub filename: usize, // Note: These aren't for file metadata
    pub filesize: usize, // filename and filesize reflect the literal display name of the columns
}

impl Table {
    pub fn attribute_without_padding(attribute: usize) -> usize {
        // The table width will be determined here.  If the longest element is
        // shorter than the title, go off the title's width.
        max(HEADER_WIDTH, attribute)
    }

    pub fn inner_computed_table_width(table_widths: &TableFormat) -> Self {
        let min_filename_width = HEADER_WIDTH;
        let min_filesize_width = HEADER_WIDTH;

        let actual_filename_width = max(
            min_filename_width + PADDING_OFFSET,
            table_widths.filename_width + PADDING_OFFSET,
        );
        let actual_filesize_width = max(
            min_filesize_width + PADDING_OFFSET,
            table_widths.filesize_width + PADDING_OFFSET,
        );

        Self {
            filename: actual_filename_width,
            filesize: actual_filesize_width,
        }
    }

    // We need to find the max length of the filesize and filepath,
    // so that we know how wide to make the table
    pub fn max_filesize_width(fileinfo: &Vec<FileInfo>) -> usize {
        let mut result: usize = 0;

        for file in fileinfo {
            let width = file.filesize() as usize;

            if width > result {
                result = width;
            }
        }

        result.separated_string().to_string().len()
    }

    pub fn max_filename_width(fileinfo: &Vec<FileInfo>) -> usize {
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
