// This file contains functions for actually displaying the table

use table::Table;
use file_info::FileInfo;

pub struct TableFormat {
    pub filename_width: usize,
    pub filesize_width: usize,
}

impl TableFormat {
    pub fn print_table(fileinfo: Vec<FileInfo>) {
        let table_widths = TableFormat {
            filename_width: Table::max_filename_width(&fileinfo),
            filesize_width: Table::max_filesize_width(&fileinfo),
        };

        let inner_width = Table::inner_computed_table_width(&table_widths);

        Self::print_header(&inner_width, &table_widths);
        Self::print_body(&fileinfo, &table_widths);
        Self::print_footer(&inner_width);
    }

    fn print_header(inner_width: &Table, table_widths: &TableFormat) {
        println!(
            "┌{}┬{}┐",
            "─".repeat(inner_width.filename),
            "─".repeat(inner_width.filesize)
        );
        println!(
            "│ {:name$} │ {:>size$} │",
            "Filename",
            "Filesize",
            name = table_widths.filename_width,
            size = table_widths.filesize_width
        );
        println!(
            "├{}┼{}┤",
            "─".repeat(inner_width.filename),
            "─".repeat(inner_width.filesize)
        );
    }

    fn print_body(fileinfo: &Vec<FileInfo>, table_widths: &TableFormat) {
        for file in fileinfo {
            println!(
                "│ {:name$} │ {:>size$} │",
                file.formatted_filepath(),
                file.formatted_filesize(),
                name = Table::attribute_without_padding(table_widths.filename_width),
                size = Table::attribute_without_padding(table_widths.filesize_width)
            );
        }
    }

    fn print_footer(inner_width: &Table) {
        println!(
            "└{}┴{}┘",
            "─".repeat(inner_width.filename),
            "─".repeat(inner_width.filesize)
        );
    }
}
