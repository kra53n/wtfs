use std::fs::{DirEntry, read_to_string};

pub fn get_lines(entry: &DirEntry) -> Result<usize, std::io::Error> {
	let stream = read_to_string(entry.path());
	match stream {
		Ok(s) => Ok(s
			.lines()
			.collect::<Vec<&str>>()
			.len()
		),
		Err(e) => Err(e),
	}
}
