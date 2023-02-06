pub mod entry;
pub use entry::Entry;

pub mod table;

pub mod statistics;

use clap::ValueEnum;

use reader;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SortOption {
    /// By path
    Path,
    /// By name
    Name,
    /// By extension
    Extension,
    /// By lines
    Lines,
    /// By size
    Size,
}

// TODO use config as function argument
pub fn run(
    reader_config: &reader::Config,
    sort: Option<SortOption>,
) -> Result<(), std::io::Error> {
    let dir_entries = reader::get_files(&reader_config)?;
    let mut entries: Vec<Entry> = dir_entries.iter()
        .map(|elem| Entry::from(elem))
        .collect();

    match sort {
        Some(sort) => match sort {
            SortOption::Path => entries.sort_by_key(|elem|
                elem.path.to_string_lossy().into_owned()),
            SortOption::Name => entries.sort_by_key(|elem|
                elem.name.to_string_lossy().into_owned()),
            SortOption::Extension => entries.sort_by_key(|elem|
                String::clone(&elem.extension)),
            SortOption::Lines => entries.sort_by_key(|elem| {
                match elem.lines {
                    Ok(lines) => lines,
                    Err(_) => 0,
                }
            }),
            SortOption::Size => entries.sort_by_key(|elem| elem.size),
        },
        _ => (),
    };

	statistics::print(&entries);
	println!();
	
    table::print(&entries);

    Ok(())
}
