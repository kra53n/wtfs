use std::fs::{DirEntry, read_to_string};

pub fn get_lines(entry: &DirEntry) -> Result<usize> {
	Ok(read_to_string(entry.path())
		.unwrap()
		.lines()
		.collect::<Vec<&str>>()
		.len()
	)
}
