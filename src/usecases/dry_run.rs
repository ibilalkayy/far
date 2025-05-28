use std::{fs::File, io::Read};

pub fn dry_run_text(taken_path: &str, from: &str, to: &str) {
    let mut content = String::new();
    File::open(taken_path)
        .expect("Err: file opening failed")
        .read_to_string(&mut content)
        .expect("Err: file reading failed");

    let new_content = content.replace(from, to);
    println!("{}", new_content);
}
