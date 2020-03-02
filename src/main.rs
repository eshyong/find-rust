extern crate clap;

use std::fs;
use std::fs::FileType;
use std::path::Path;

use clap::{App, Arg};
use either::Either;
use either::Either::{Left, Right};
use regex::Regex;
use std::ffi::OsStr;

// sigh
#[derive(Clone, Copy)]
enum _FileType {
    File,
    Directory,
    Symlink,
    Any,
}

fn main() {
    let matches = App::new("find")
        .version("0.1")
        .author("Eric S. <eric.shyong@gmail.com>")
        .about("Replacement for find, written in Rust")
        .arg(Arg::with_name("pattern")
            .help("A shell pattern to match file names against. Uses regex internally")
            .required(true)
            .index(1))
        .arg(Arg::with_name("path")
            .help("Sets the path to search")
            .long("path")
            .takes_value(true))
        .arg(Arg::with_name("type")
            .help("The type of file to search for. Can be \"file\", \"dir\", or \"symlink\"")
            .long("type")
            .validator(validate_file_type)
            .takes_value(true))
        .get_matches();

    let str_or_regex = str_to_regex_or_literal(
        matches.value_of("pattern").unwrap());
    let search_type = convert_str_to_file_type(
        matches.value_of("type").unwrap_or("any"));
    let path_str = matches.value_of("path").unwrap_or(".");

    find_impl(Path::new(path_str), &str_or_regex, search_type);
}

fn validate_file_type(v: String) -> Result<(), String> {
    match v.as_str() {
        "file" | "dir" | "symlink" | "any" => Ok(()),
        _ => Err(String::from("should be one of \"file\", \"dir\", or \"symlink\""))
    }
}

fn str_to_regex_or_literal(pattern: &str) -> Either<String, Regex> {
    return if contains_shell_patterns(pattern) {
        // Replace shell pattern metacharacters with their regex equivalents
        // reference: https://www.gnu.org/software/findutils/manual/html_node/find_html/Shell-Pattern-Matching.html
        let compiled_pattern = pattern.clone()
            .replace("*", ".*")
            .replace("?", ".");

        match Regex::new(compiled_pattern.as_ref()) {
            Ok(re) => Right(re),
            Err(e) => panic!(e),
        }
    } else {
        Left(pattern.clone().to_string())
    };

}

fn contains_shell_patterns(s: &str) -> bool {
    s.contains("*") || s.contains("?")
}

fn convert_str_to_file_type(s: &str) -> _FileType {
    match s {
        "file" => _FileType::File,
        "dir" => _FileType::Directory,
        "symlink" => _FileType::Symlink,
        "any" => _FileType::Any,
        _ => panic!("shouldn't happen: unknown filetype {}", s),
    }
}

fn find_impl(path: &Path, pattern: &Either<String, Regex>, search_type: _FileType) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    let curr_file = entry.path();
                    let full_path = curr_file.to_str().unwrap();
                    let file_name = curr_file
                        .file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or("");
                    match pattern {
                        Left(s) => {
                            if s == file_name && file_type_matches(file_type, search_type) {
                                println!("{}", full_path);
                            }
                        },
                        Right(r) => {
                            if r.is_match(file_name) &&
                                file_type_matches(file_type, search_type) {
                                println!("{}", full_path);
                            }
                        }
                    }
                    if file_type.is_dir() {
                        find_impl(&curr_file, pattern, search_type);
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
        _FileType::Any => true,
    }
}
