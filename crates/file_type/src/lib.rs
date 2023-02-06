pub mod constants;
pub use constants::FILE_TYPES;

use std::path::Path;
use std::ffi::OsStr;

pub fn get_file_type(file: &Path) -> &str {
    let extension = file.extension().and_then(OsStr::to_str);
    if let Some(ext) =  extension {
        if let Some(val) = FILE_TYPES.get(ext).cloned() {
            return val;
        };
    }
    "-"
}
