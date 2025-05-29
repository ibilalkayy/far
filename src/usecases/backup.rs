use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn file_backup(backup_file: &str, target: &str) {
    let home_dir = dirs::home_dir().expect("Err: failed to get the home directory");
    let joined_dir = home_dir.join("fara");
    let merge_path = joined_dir.join(backup_file);

    if !joined_dir.exists() {
        fs::create_dir(&joined_dir).expect("Err: failed to create the home directory");
    }

    if !merge_path.exists() {
        let mut data_file = File::create_new(&merge_path).expect("Err: failed to create the file");
        let mut data_result = File::open(target).expect("Err: failed to open the file");
        let mut file_content = String::new();

        data_result.read_to_string(&mut file_content).unwrap();
        data_file
            .write(file_content.as_bytes())
            .expect("Err: failed to write into the file");

        println!(
            "Backup data is successfully saved in the {:?} file",
            merge_path
        );
    } else {
        eprintln!("Err: {:?} file already exists", merge_path);
    }
}
