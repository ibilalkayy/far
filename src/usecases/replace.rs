use std::fs;

pub fn replace_text(path: String, from: String, to: String) {
    let content = fs::read_to_string(path.clone()).unwrap();
    let file_content = content.replace(&from, &to);
    let status = fs::write(path, file_content);
    match status {
        Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
        Err(error) => eprintln!("Failure occurred while replacing the text: {}", error)
    }
}
