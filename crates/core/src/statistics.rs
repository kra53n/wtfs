use std::collections::HashMap;

use crate::Entry;

#[derive(Debug)]
struct Statistics {
    file_num: u64,
    file_types_by_num: HashMap<String, u8>,
    file_types_by_lines: HashMap<String, usize>,
    lines: usize,
}

fn get(entries: &Vec<Entry>) -> Statistics {
	let mut file_num = 0;
	let mut file_types_by_num: HashMap<String, u8> = HashMap::new();
	let mut file_types_by_lines: HashMap<String, usize> = HashMap::new();
	let mut lines = 0;

	for entry in entries {
		file_num += 1;

		match file_types_by_num.get_mut(&entry.extension) {
			Some(ext) => *ext += 1,
			None => {
				file_types_by_num.insert(String::from(&entry.extension), 1);
				()
			},
		};

		match file_types_by_lines.get_mut(&entry.extension) {
			Some(ext) => *ext += *entry.lines.as_ref().unwrap_or_else(|_| &0),
			None => {
				file_types_by_lines.insert(
					String::from(&entry.extension),
					*entry.lines.as_ref().unwrap_or_else(|_| &0)
				);
				()
			},
		};

		lines += *entry.lines.as_ref().unwrap_or_else(|_| &0);
	}

	Statistics {
		file_num,
		file_types_by_num,
		file_types_by_lines,
		lines,
	}
}

pub fn print(entries: &Vec<Entry>) {
	println!("{:?}", get(&entries));
}
