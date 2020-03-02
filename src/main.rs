extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::path::Path;
use std::fs::FileType;

// sigh
#[derive(Clone, Copy)]
enum _FileType {
    File,
    Directory,
    Symlink,
}

fn main() {
    let matches = App::new("find")
        .version("0.1")
        .author("Eric S. <eric.shyong@gmail.com>")
        .about("Replacement for find, written in Rust")
        .arg(Arg::with_name("name")
            .help("Name of file or directory to search for")
            .required(true)
            .index(1))
        .arg(Arg::with_name("path")
            .help("Sets the path to search")
            .long("path"))
        .arg(Arg::with_name("type")
            .long("type")
            .validator(validate_file_type)
            .takes_value(true))
        .get_matches();

    let search_str = matches.value_of("name").unwrap();
    let search_type = convert_str_to_file_type(
        matches.value_of("type").unwrap_or("file"));
    let path_str = matches.value_of("path").unwrap_or(".");

    find_impl(Path::new(path_str), search_str, search_type);
}

fn validate_file_type(v: String) -> Result<(), String> {
    match v.as_str() {
        "file" | "dir" | "symlink" => Ok(()),
        _ => Err(String::from("should be one of \"file\", \"dir\", or \"symlink\""))
    }
}

fn convert_str_to_file_type(s: &str) -> _FileType {
    match s {
        "file" => _FileType::File,
        "dir" => _FileType::Directory,
        "symlink" => _FileType::Symlink,
        _ => panic!("shouldn't happen: unknown filetype {}", s),
    }
}

fn find_impl(path: &Path, search_str: &str, search_type: _FileType) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    let entry_path = entry.path();
                    let file_name = entry_path.to_str().unwrap_or("");
                    if file_name.contains(search_str) &&
                        file_type_matches(file_type, search_type) {
                        println!("{}", file_name);
                    }
                    if file_type.is_dir() {
                        find_impl(&entry_path, search_str, search_type);
                    }
                }
            }
        }
    }
}

fn file_type_matches(file_type: FileType, search_type: _FileType) -> bool {
    match search_type {
        _FileType::File => file_type.is_file(),
        _FileType::Directory => file_type.is_dir(),
        _FileType::Symlink => file_type.is_symlink(),
    }
}
