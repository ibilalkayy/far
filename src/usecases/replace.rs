use std::fs;

use regex::RegexBuilder;

pub fn replace_text(path: &str, from: &str, to: &str, ignore_case: bool) {
    let content = fs::read_to_string(path).unwrap();

    let file_content = if ignore_case {
        let replaced = RegexBuilder::new(from)
            .case_insensitive(true)
            .build()
            .unwrap();
        replaced.replace_all(&content, to).to_string()
    } else {
        content.replace(from, to)
    };

    let status = fs::write(path, file_content);
    match status {
        Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
        Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
    }
}

pub fn replace_text_in_file(
    taken_path: &str,
    target_path: &str,
    from: &str,
    to: &str,
    ignore_case: bool,
) {
    let content = fs::read_to_string(taken_path).unwrap();

    let file_content = if ignore_case {
        let replaced = RegexBuilder::new(from)
            .case_insensitive(true)
            .build()
            .unwrap();
        replaced.replace_all(&content, to).to_string()
    } else {
        content.replace(from, to)
    };

    let status = fs::write(target_path, file_content);
    match status {
        Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
        Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
    }
}
