use std::cmp;
use crate::Entry;

struct Spaces {
    between_cols: usize,
    path: usize,
    lines: usize,
    size: usize,
    extension: usize,
}

fn get_max_path(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .max_by_key(|elem| elem.path.len())
        .unwrap()
        .path
        .len()
}

fn get_max_extension(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .max_by_key(|elem| elem.extension.len())
        .unwrap()
        .name
        .len()
}

fn get_max_lines(entries: &Vec<Entry>) -> usize {
    let res = &entries
        .iter()
        .max_by_key(|elem| {
            match elem.lines {
                Ok(lines) => lines.to_string().len(),
                Err(_) => 0,
            }
        })
        .unwrap()
        .lines;

    match &res {
        Ok(lines) => lines.to_string().len(),
        Err(_) => 0,
    }
}

fn get_max_size(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .max_by_key(|elem| elem.size.to_string().len())
        .unwrap()
        .size
        .to_string()
        .len()
}

fn print_spaces(num: usize) {
    for _ in 0..num {
        print!(" ");
    }
}

fn print_row(elems: &mut Vec<String>, spaces: &Spaces) {
    let path = elems.pop().unwrap();
    let extension = elems.pop().unwrap();
    let lines = elems.pop().unwrap();
    let size = elems.pop().unwrap();

    print!("{}", path);
    print_spaces(spaces.path - path.len() + spaces.between_cols);

    print!("{}", extension);
    print_spaces(spaces.extension - extension.len() + spaces.between_cols);

    print!("{}", lines);
    print_spaces(spaces.lines - lines.len() + spaces.between_cols);
    print_spaces(spaces.between_cols);

    print!("{}", size);
    print_spaces(spaces.size - size.len() + spaces.between_cols);

    println!();
}

pub fn print(entries: &Vec<Entry>) {
    let spaces = Spaces {
        between_cols: 3,
        path: cmp::max(get_max_path(&entries), "path".len()),
        extension: cmp::max(get_max_extension(&entries), "extension".len()),
        lines: cmp::max(get_max_lines(&entries), "lines".len()),
        size: cmp::max(get_max_size(&entries), "size".len()),
    };

    print_row(
        &mut vec![
            String::from("SIZE"),
            String::from("LINES"),
            String::from("EXTENSION"),
            String::from("PATH"),
        ],
        &spaces,
    );
    println!();

    for entry in entries {
        print_row(
            &mut vec![
                entry.size.to_string(),
                match entry.lines {
                    Ok(lines) => lines.to_string(),
                    Err(_) => String::from("-"),
                },
                entry.extension.to_string(),
                entry.path.to_string_lossy().into_owned(),
            ],
            &spaces,
        );
    }
}
