extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("find")
        .version("0.1")
        .author("Eric S. <eric.shyong@gmail.com>")
        .about("Replacement for find, written in Rust")
        .arg(Arg::with_name("path")
            .help("Sets the path to search")
            .required(true)
            .index(1))
        .arg(Arg::with_name("name")
                .long("name")
                .required(true)
                .takes_value(true))
        .get_matches();

    let search_str = matches.value_of("name").unwrap();
    let path = matches.value_of("path").unwrap();
    find_impl(Path::new(path), search_str);
}

fn find_impl(path: &Path, search_str: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                match entry.path().to_str() {
                    Some(entry_path) => {
                        if entry_path.contains(search_str) {
                            println!("{}", entry_path);
                        }
                    }
                    None => (),
                }
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        find_impl(&entry.path(), search_str);
                    }
                }
            }
        }
    }
}
