use crate::{
    cli::command::command::Far,
    usecases::{dry_run::dry_run_text, find::find_text, regex::find_regex, replace::replace_text},
};
use std::{fs, fs::File};

impl Far {
    pub fn control_args(self) {
        if self.dry_run {
            self.handle_dry_run();
        } else if let Some(path) = &self.target {
            self.find_the_text(path);
        } else {
            eprintln!("Err: use either --target to write or --dry-run to preview");
        }
    }

    fn handle_dry_run(self) {
        if let Some(path) = &self.target {
            if let Some(find) = &self.find {
                dry_run_text(path, find, &self.replace);
            } else if let Some(regex) = &self.regex {
                if let Some(regex_found) = find_regex(&regex, &path) {
                    dry_run_text(path, &regex_found, &self.replace);
                } else {
                    eprintln!("Err: '{}' is not found in the given file", regex);
                }
            } else {
                eprintln!("Err: gmail value is not provided to find the text");
            }
        } else {
            eprintln!("Err: dry run needs a --target file path to preview");
        }
    }

    fn handle_replacing(&self, path: &String, find_text: &String) {
        if self.backup.len() > 0 {
            if self.confirm {
                self.create_file();
                replace_text(path, find_text, &self.replace);
            } else {
                eprintln!("Err: use --confirm to replace the text");
            }
        } else {
            eprintln!("Err: please take the backup first before replacing the data");
        }
    }

    fn find_the_text(&self, path: &String) {
        if let Some(find) = &self.find {
            let text_found = find_text(find, path);
            if text_found {
                self.handle_replacing(path, find);
            } else {
                eprintln!("Err: '{}' is not found in the given file", find);
            }
        } else if let Some(regex) = &self.regex {
            if let Some(regex_found) = find_regex(&regex, &path) {
                self.handle_replacing(path, &regex_found);
            } else {
                eprintln!("Err: '{}' is not found in the given file", regex);
            }
        } else {
            eprintln!("Err: value is not provided to find the text");
        }
    }

    fn create_file(&self) {
        let home_dir = dirs::home_dir().expect("Err: failed to get the home directory");
        let joined_dir = home_dir.join("far");

        if !joined_dir.exists() {
            fs::create_dir(&joined_dir).expect("Err: failed to create the home directory");
        }

        let merge_path = joined_dir.join(&self.backup);
        File::create_new(merge_path).expect("Err: failed to create the file");
    }
}
