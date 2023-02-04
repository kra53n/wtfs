use std::process;

use clap::Parser;

use reader;
use core::{run, SortOption};

/// Utility for printing file statistic of project
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // Path to dir
    path: Option<String>,

    /// Sort by columns
    #[arg(short, long)]
    #[arg(value_enum)]
    sort: Option<SortOption>,

    #[arg(short, long)]
    recursively: bool,

    #[arg(short='a', long)]
    include_hidden_entries: bool,

    #[arg(short='I', long)]
    ignore_specific_entry: Option<Vec<String>>,

    #[arg(short='i', long)]
    include_specific_entry: Option<Vec<String>>,
}


fn main() {
    let args = Args::parse();

    let reader_config = reader::Config {
        path: args.path.unwrap_or_else(|| String::from(".")),
        ignore_hidden_entries: !args.include_hidden_entries,
        ignore_specific_entries: args.ignore_specific_entry.unwrap_or_else(|| vec![]),
        include_specific_entries: args.include_specific_entry.unwrap_or_else(|| vec![]),
        recursively: args.recursively,
    };

    if let Err(e) = run(&reader_config, args.sort) {
        println!("Applicaton error: {}", e);
        process::exit(1);
    }
}
