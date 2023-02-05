use std::collections::HashMap;

use crate::Entry;

struct Statistics {
    file_num: u64,
    file_types_by_num: HashMap<String, u8>,
    file_types_by_lines: HashMap<String, u8>,
    lines: u64,
}

fn get(entries: &Vec<Entry>) {

}

pub fn print(entries: &Vec<Entry>) {
}
