pub mod read;
pub use read::{Config, get_files};

pub mod operations;
pub use operations::get_lines;

#[cfg(test)]
mod tests {
	use super::read::*;

	use std::fs::DirEntry;
	use std::ffi::OsString;

	fn get_filenames(entries: Vec<DirEntry>) -> Vec<OsString> {
		let mut res: Vec<OsString> = vec![];

		for entry in entries {
			res.push(entry.file_name());
		}

		res
	}

	#[test]
	fn ignoring_hidden_entries() {
		let config = Config {
			path: String::from("test"),
			ignore_hidden_entries: true,
			ignore_specific_entries: vec![],
			include_specific_entries: vec![],
			recursively: false,
		};

		assert_eq!(
			get_filenames(get_files(&config)),
			vec![OsString::from("dont_ignore_me.txt")]
		);
	}

	#[test]
	fn ignore_specific_entries() {
		let config = Config {
			path: String::from("test"),
			ignore_hidden_entries: true,
			ignore_specific_entries: vec![String::from("dont_ignore_me.txt")],
			include_specific_entries: vec![],
			recursively: false,
		};

		assert_eq!(get_filenames(get_files(&config)), Vec::<OsString>::new());
	}

	#[test]
	fn include_specific_entries() {
		let config = Config {
			path: String::from("test"),
			ignore_hidden_entries: true,
			ignore_specific_entries: vec![],
			include_specific_entries: vec![String::from(".ignore_this_file")],
			recursively: false,
		};

		let mut res = get_filenames(get_files(&config));
		res.sort();

		assert_eq!(
			res,
			vec![
				OsString::from(".ignore_this_file"),
				OsString::from("dont_ignore_me.txt"),
			],
		);
	}

	#[test]
	fn recursively() {
		let config = Config {
			path: String::from("test"),
			ignore_hidden_entries: true,
			ignore_specific_entries: vec![],
			include_specific_entries: vec![],
			recursively: true,
		};

		let mut res = get_filenames(get_files(&config));
		res.sort();

		assert_eq!(
			res,
			vec![
				OsString::from("dont_ignore_me.txt"),
				OsString::from("file2"),
				OsString::from("file3"),
			],
		);
	}
}
