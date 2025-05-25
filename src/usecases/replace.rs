use std::fs;

pub fn replace_text(path: &str, from: &str, to: &str) {
    let content = fs::read_to_string(path).unwrap();
    let file_content = content.to_lowercase().replace(from, to);
    let status = fs::write(path, file_content);
    match status {
        Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
        Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
    }
}
