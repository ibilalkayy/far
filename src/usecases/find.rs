use std::{fs::File, io::Read};

pub fn find_text(text: &str, path: &str) -> bool {
    let mut file_path = File::open(path).expect("Err: failed to open the file");
    let mut file_content = String::new();

    file_path.read_to_string(&mut file_content).unwrap();
    if file_content.contains(text) {
        return true;
    } else {
        return false;
    }
}
