use regex::Regex;
use std::{fs::File, io::Read};

pub fn find_regex(expression: &str, path: &str) -> Option<String> {
    let re = Regex::new(&expression).unwrap();
    let mut file_path = File::open(path).expect("Err: failed to open the file");
    let mut file_content = String::new();

    file_path.read_to_string(&mut file_content).unwrap();
    if let Some(m) = re.find(&file_content) {
        return Some(m.as_str().to_string());
    }
    None
}
