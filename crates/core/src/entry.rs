use std::path::Path;
use std::fs::DirEntry;
use std::ffi::OsString;

use reader::get_lines;
use file_type::get_file_type;

#[derive(Debug)]
pub struct Entry {
    pub name: OsString,
    pub path: OsString,
    pub extension: String,
    pub lines: Result<usize, std::io::Error>,
    pub size: u64,
}

impl From<&DirEntry> for Entry {
    fn from(dir_entry: &DirEntry) -> Self {
        let name = OsString::from(dir_entry.file_name());
        let path = Path::new(&name);

        Self {
            name: OsString::from(path.file_name().unwrap()),
            path: OsString::from(dir_entry.path()),
            extension: String::from(get_file_type(path)),
            lines: get_lines(&dir_entry),
            size: dir_entry.metadata().unwrap().len(),
        }
    }
}
