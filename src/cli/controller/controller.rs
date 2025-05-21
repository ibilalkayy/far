use crate::{
    cli::command::command::Far,
    usecases::{dry_run::dry_run_text, find::find_text, regex::find_regex, replace::replace_text},
};
use std::{fs, fs::File};

impl Far {
    pub fn control_args(self) {
        if self.target.len() > 0 {
            self.find_the_text(&self.target);
        } else {
            eprintln!("Err: --target flag is required to replace the text");
        }
    }

    fn find_the_text(&self, path: &String) {
        if let Some(find) = &self.find {
            let text_found = find_text(find, path);
            if text_found {
                self.handle_options(path, find);
            } else {
                eprintln!("Err: '{}' is not found in the given file", find);
            }
        } else if let Some(regex) = &self.regex {
            if let Some(regex_found) = find_regex(&regex, &path) {
                self.handle_options(path, &regex_found);
            } else {
                eprintln!("Err: '{}' is not found in the given file", regex);
            }
        } else {
            eprintln!("Err: value is not provided to find the text");
        }
    }

    fn handle_options(&self, path: &String, find_text: &String) {
        if self.confirm {
            replace_text(path, find_text, &self.replace);
        } else if self.dry_run {
            self.handle_dry_run();
        } else if let Some(backup_file) = &self.backup {
            self.handle_file_backup(backup_file);
        } else {
            eprintln!("Err: use either --confirm to replace or --dry-run to preview the text");
        }
    }

    fn handle_dry_run(&self) {
        if self.target.len() > 0 {
            if let Some(find) = &self.find {
                dry_run_text(&self.target, find, &self.replace);
            } else if let Some(regex) = &self.regex {
                if let Some(regex_found) = find_regex(&regex, &self.target) {
                    dry_run_text(&self.target, &regex_found, &self.replace);
                } else {
                    eprintln!("Err: '{}' is not found in the given file", regex);
                }
            } else {
                eprintln!("Err: text is not provided to find it");
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
            File::create_new(&merge_path).expect("Err: failed to create the file");
            println!("Backup data is saved in the {:?} file", merge_path);
        } else {
            eprintln!("Err: {:?} file already exists", merge_path);
        }
    }
}
