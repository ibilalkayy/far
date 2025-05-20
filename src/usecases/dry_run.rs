use std::fs;

pub fn dry_run_text(path: &str, from: &str, to: &str) {
    let content = fs::read_to_string(path).unwrap();
    let file_content = content.replace(from, to);
    println!("{}", file_content);
}
