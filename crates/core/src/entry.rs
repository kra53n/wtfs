use std::fs::DirEntry;
use std::ffi::OsString;

use reader::get_lines;

#[derive(Debug)]
pub struct Entry {
    pub name: OsString,
    pub path: OsString,
    pub lines: Result<usize, std::io::Error>,
    pub size: u64,
}

impl From<&DirEntry> for Entry {
    fn from(dir_entry: &DirEntry) -> Self {
        Self {
            name: OsString::from(dir_entry.file_name()),
            path: OsString::from(dir_entry.path()),
            lines: get_lines(&dir_entry),
            size: dir_entry.metadata().unwrap().len(),
        }
    }
}
