use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn ignore_case<'a>(find_data: &str, path: &str) -> String {
    let file_text = File::open(path).expect("Err: failed to open the file");
    let reader = BufReader::new(file_text);

    for line in reader.lines() {
        let line = line.expect("Err: failed to read the line");

        for word in line.split_whitespace() {
            if word.eq_ignore_ascii_case(find_data) {
                return word.to_string();
            }
        }
    }
    return "".to_string();
}
