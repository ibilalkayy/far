use std::fs;

pub fn dry_run_text(path: String, from: String, to: String) {
    let content = fs::read_to_string(path.clone()).unwrap();
    let file_content = content.replace(&from, &to);
    println!("{}", file_content);
}
