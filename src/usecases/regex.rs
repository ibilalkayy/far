use regex::Regex;
use std::fs;
use walkdir::WalkDir;

pub fn find_regex(expression: &str, path: &str) -> Option<String> {
    let re = Regex::new(&expression).unwrap();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();

            if let Some(m) = re.find(&content) {
                return Some(m.as_str().to_string());
            }
        }
    }
    None
}
