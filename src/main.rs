extern crate clap;

use std::path::Path;

use clap::{App, Arg};

mod finder;

fn validate_file_type(v: String) -> Result<(), String> {
    match v.as_str() {
        "file" | "dir" | "symlink" | "any" => Ok(()),
        _ => Err(String::from("should be one of \"file\", \"dir\", or \"symlink\""))
    }
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

    let str_or_regex = matches.value_of("pattern").unwrap();
    let file_type = matches.value_of("type").unwrap_or("any");
    let path_str = matches.value_of("path").unwrap_or(".");

    finder::Finder::new(str_or_regex, file_type).search(Path::new(path_str));
}

