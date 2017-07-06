// This file contains functions for calculating table offsets and widths.
use std::cmp::max;
use file_info::FileInfo;
use table_format::TableFormat;

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

    // fmt doens't handle on-standard chars correctly.
    // This is needed to correctly report the length of a string
    pub fn normalized_filename_padding(attribute: usize, file: &FileInfo) -> String {
        let normalized_padding = Self::attribute_without_padding(attribute) -
            file.normalized_filename_length();

        " ".repeat(normalized_padding)
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
    pub fn max_filesize_width(fileinfo: &[FileInfo]) -> usize {
        fileinfo
            .into_iter()
            .map(|f| f.formatted_filesize().len())
            .max()
            .unwrap_or(0)
    }

    pub fn max_filename_width(fileinfo: &[FileInfo]) -> usize {
        fileinfo
            .into_iter()
            .map(|f| f.normalized_filename_length())
            .max()
            .unwrap_or(0)
    }
}
