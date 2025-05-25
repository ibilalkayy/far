use crate::{
    cli::command::command::Far,
    usecases::{
        dry_run::dry_run_text, find::find_txt, ignore_case::ignore_case, regex::find_regex,
        replace::replace_text,
    },
};
use std::{
    fs::{self, File},
    io::{Read, Write},
};

impl Far {
    pub fn control_args(self) {
        if self.target.trim().is_empty() {
            eprintln!("Err: --target flag is required to replace, preview, or backup text");
            return;
        }

        if self.backup.is_some()
            && (self.find.is_some() || self.replace.is_some() || self.dry_run || self.confirm)
        {
            eprintln!(
                "Err: --backup cannot be used with --find, --replace, --dry-run, or --confirm"
            );
            return;
        }

        if self.extension.is_some()
            && (self.find.is_some() || self.replace.is_some() || self.dry_run || self.confirm)
        {
            eprintln!(
                "Err: --extension cannot be used with --find, --replace, --dry-run, or --confirm"
            );
            return;
        }

        if let Some(backup_file) = &self.backup {
            if let Some(ext) = &self.extension {
                if let Some(backup_file) = &self.backup {
                    let mut text_ext = backup_file.split(".");
                    text_ext.next().unwrap();
                    let matched_word = text_ext.next().unwrap();
                    let fetched_word = ext.replace(".", "");

                    if fetched_word == matched_word {
                        self.handle_file_backup(backup_file);
                    } else {
                        println!(
                            "Err: '{}' file extension didn't match with the extension that you gave",
                            backup_file
                        );
                    }
                }
            } else {
                self.handle_file_backup(backup_file);
            }
            return;
        }

        if self.confirm || self.dry_run || self.ignore_case {
            if self.find.is_none() && self.regex.is_none() {
                eprintln!("Err: --confirm or --dry-run requires either --find or --regex");
                return;
            }
            if self.replace.is_none() {
                eprintln!("Err: --confirm or --dry-run requires --replace");
                return;
            }

            self.find_text(&self.target);
            return;
        }

        eprintln!("Err: no valid operation selected. Use --backup, --dry-run, or --confirm");
    }

    fn find_text(&self, path: &String) {
        if let Some(find) = &self.find {
            if self.ignore_case {
                let result_word = ignore_case(find, path);
                self.handle_options(path, &result_word.to_lowercase(), true);
                return;
            }

            let text_found = find_txt(find, path);
            if text_found {
                self.handle_options(path, find, false);
            } else {
                eprintln!("Err: '{}' is not found in the given file", find);
            }
        } else if let Some(regex) = &self.regex {
            if let Some(regex_text) = find_regex(regex, path) {
                self.handle_options(path, &regex_text, false);
            } else {
                eprintln!("Err: '{}' is not found in the given file", regex);
            }
        }
    }

    fn handle_options(&self, path: &String, find_text: &String, case: bool) {
        let replace_txt = self.replace.as_ref().unwrap();

        if self.confirm {
            replace_text(path, find_text, replace_txt, case);
        } else if self.dry_run {
            self.handle_dry_run(replace_txt);
        }
    }

    fn handle_dry_run(&self, replace_txt: &String) {
        if self.target.len() > 0 {
            if let Some(find) = &self.find {
                dry_run_text(&self.target, find, replace_txt);
            } else if let Some(regex) = &self.regex {
                if let Some(regex_found) = find_regex(&regex, &self.target) {
                    dry_run_text(&self.target, &regex_found, replace_txt);
                } else {
                    eprintln!("Err: '{}' is not found in the given file", regex);
                }
            } else {
                eprintln!("Err: use either --find or --regex flag to find the text");
            }
        }
    }

    fn handle_file_backup(&self, backup_file: &String) {
        let home_dir = dirs::home_dir().expect("Err: failed to get the home directory");
        let joined_dir = home_dir.join("far");
        let merge_path = joined_dir.join(backup_file);

        if !joined_dir.exists() {
            fs::create_dir(&joined_dir).expect("Err: failed to create the home directory");
        }

        if !merge_path.exists() {
            let mut data_file =
                File::create_new(&merge_path).expect("Err: failed to create the file");
            let mut data_result = File::open(&self.target).expect("Err: failed to open the file");
            let mut file_content = String::new();

            data_result.read_to_string(&mut file_content).unwrap();
            data_file
                .write(file_content.as_bytes())
                .expect("Err: faile to write into the file");

            println!(
                "Backup data is successfully saved in the {:?} file",
                merge_path
            );
        } else {
            eprintln!("Err: {:?} file already exists", merge_path);
        }
    }
}
