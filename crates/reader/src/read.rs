use std::fs::{self, DirEntry, ReadDir};
use std::collections::LinkedList;

/// If ignore and include entries controversery each other, the ignore
/// behaviour will choosen.
pub struct Config {
	pub path: String,
	pub ignore_hidden_entries: bool,
	pub ignore_specific_entries: Vec<String>,
	pub include_specific_entries: Vec<String>,
	pub recursively: bool,
}

fn is_hidden_entry(entry: &DirEntry) -> bool {
	let name = entry.file_name().into_string().unwrap();
	let mut name = name.char_indices();
	match name.next() {
		Some(c) => c.1 == '.',
		None => false,
	}
}

fn include_entry(config: &Config, entry: &DirEntry) -> bool {
	let name = entry.file_name().into_string().unwrap();

	if config.ignore_specific_entries.contains(&name) {
		return false;
	}
	if config.include_specific_entries.contains(&name) {
		return true;
	}

	if config.ignore_hidden_entries && is_hidden_entry(&entry) {
		return false;
	}

	true
}

pub fn get_files(config: &Config) -> Result<Vec<DirEntry>, std::io::Error> {
	let curr_dir = fs::read_dir(&config.path)?;
	let mut dirs: LinkedList<ReadDir> = LinkedList::new();
	dirs.push_back(curr_dir);
	let mut res: Vec<DirEntry> = vec![];

	while !dirs.is_empty() {
		let dir = dirs.pop_front().unwrap();

		for entry in dir {
			let entry = entry.unwrap();
			let file_type = entry.file_type().unwrap();

			if !include_entry(&config, &entry) {
				continue;
			}

			if file_type.is_dir() {
				let read_dir = fs::read_dir(&entry.path()).unwrap();
				dirs.push_back(read_dir);
			} else if file_type.is_file() {
				res.push(entry);
			}
		}

		if !config.recursively {
			return Ok(res);
		}
	}
	Ok(res)
}
