pub struct Table;

impl Table {
    pub fn print_table(fileinfo: &FileInfo) {
        let filename_width: usize = string_table_width(fileinfo);
        let filesize_width: usize = numeric_table_width(fileinfo);
        let inner_width = inner_computed_table_width(filename_width, filesize_width);
    }

    fn print_header() {}

    fn print_footer() {
        println!(
            "└{}┴{}┘",
            "─".repeat(inner_width.filename),
            "─".repeat(inner_width.filesize)
        );
    }
}
