use core::{Entry, print_table};
use reader;

fn main() {
    let reader_config = reader::Config {
        path: String::from("."),
        ignore_hidden_entries: true,
        ignore_specific_entries: vec![],
        include_specific_entries: vec![],
        recursively: false,
    };
    let dir_entries = reader::get_files(&reader_config);
    let entries = dir_entries.iter().map(|elem| Entry::from(elem)).collect();
    print_table(&entries);
}
