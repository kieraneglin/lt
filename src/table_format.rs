use table::Table;
use file_info::FileInfo;

pub struct TableFormat;

impl TableFormat {
    pub fn print_table(fileinfo: Vec<FileInfo>) {
        let filename_width = Table::string_table(&fileinfo);
        let filesize_width = Table::numeric_table(&fileinfo);
        let inner_width = Table::inner_computed_table(filename_width, filesize_width);

        Self::print_header(&inner_width, filename_width, filesize_width);
        Self::print_body(fileinfo, filename_width, filesize_width);
        Self::print_footer(&inner_width);
    }

    fn print_header(inner_width: &Table, filename_width: usize, filesize_width: usize) {
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
    }

    fn print_body(fileinfo: Vec<FileInfo>, filename_width: usize, filesize_width: usize) {
        for file in fileinfo {
            println!(
                "│ {:name$} │ {:>size$} │",
                file.formatted_filepath(),
                file.formatted_filesize(),
                name = Table::attribute_without_padding(filename_width),
                size = Table::attribute_without_padding(filesize_width)
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
