use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

// pub fn replace_text(path: &str, from: &str, to: &str) {
//     let mut content = String::new();
//     File::open(path)
//         .expect("Err: file opening failed")
//         .read_to_string(&mut content)
//         .expect("Err: file reading failed");

//     let new_content = content.replace(from, to);

//     let mut file = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open(path)
//         .expect("Err: file opening for writing failed");

//     match file.write_all(new_content.as_bytes()) {
//         Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
//         Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
//     }
// }

// pub fn replace_text_two(from_path: &str, to_path: &str, from: &str, to: &str) {
//     let mut content = String::new();
//     File::open(from_path)
//         .expect("Err: file opening failed")
//         .read_to_string(&mut content)
//         .expect("Err: file reading failed");

//     let new_content = content.replace(from, to);

//     let mut file = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(true)
//         .open(to_path)
//         .expect("Err: file opening for writing failed");

//     match file.write_all(new_content.as_bytes()) {
//         Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
//         Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
//     }
// }

pub fn replace_text(from_path: &str, to_path: &str, from: &str, to: &str) {
    let mut content = String::new();
    File::open(from_path)
        .expect("Err: file opening failed")
        .read_to_string(&mut content)
        .expect("Err: file reading failed");

    let new_content = content.replace(from, to);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(to_path)
        .expect("Err: file opening for writing failed");

    match file.write_all(new_content.as_bytes()) {
        Ok(_) => println!("'{}' is successfully replaced with '{}'", from, to),
        Err(error) => eprintln!("Failure occurred while replacing the text: {}", error),
    }
}
