use std::ffi::OsStr;
use std::fs;
use std::fs::FileType as OsFileType;
use std::path::Path;

use regex::Regex;

#[derive(Clone, Copy)]
enum FileType {
    File,
    Directory,
    Symlink,
    Any,
}

pub struct Finder<'a> {
    pattern: &'a str,
    pattern_regex: Option<Regex>,
    file_type: FileType,
}

impl Finder<'_> {
    pub fn new<'a>(pattern: &'a str, file_type: &str) -> Finder<'a> {
        let pattern_regex = if contains_shell_patterns(pattern) {
            Some(str_to_regex(pattern))
        } else {
            None
        };

        Finder {
            pattern,
            pattern_regex,
            file_type: match file_type {
                "file" => FileType::File,
                "dir" => FileType::Directory,
                "symlink" => FileType::Symlink,
                _ => FileType::Any,
            },
        }
    }

    pub fn search(&self, path: &Path) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        let curr_path = entry.path();
                        let full_path = curr_path.to_str().unwrap();
                        let file_name = curr_path
                            .file_name()
                            .unwrap_or(OsStr::new(""))
                            .to_str()
                            .unwrap_or("");
                        let filename_matches = match &self.pattern_regex {
                            Some(re) => re.is_match(file_name),
                            // TBD whether this is the "right" behavior. `find` does this but
                            // it's not particularly intuitive
                            None => file_name == self.pattern,
                        };
                        if filename_matches && self.file_type_matches(file_type) {
                            println!("{}", full_path);
                        }

                        if file_type.is_dir() {
                            self.search(&curr_path);
                        }
                    }
                }
            }
        }
    }

    fn file_type_matches(&self, file_type: OsFileType) -> bool {
        match self.file_type {
            FileType::File => file_type.is_file(),
            FileType::Directory => file_type.is_dir(),
            FileType::Symlink => file_type.is_symlink(),
            FileType::Any => true,
        }
    }
}

fn str_to_regex(pattern: &str) -> Regex {
    // Replace shell pattern metacharacters with their regex equivalents
    // reference: https://www.gnu.org/software/findutils/manual/html_node/find_html/Shell-Pattern-Matching.html
    let compiled_pattern = pattern.clone()
        .replace("*", ".*")
        .replace("?", ".");

    match Regex::new(compiled_pattern.as_ref()) {
        Ok(re) => re,
        Err(e) => panic!(e),
    }
}

fn contains_shell_patterns(s: &str) -> bool {
    s.contains("*") || s.contains("?")
}
