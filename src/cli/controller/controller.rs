use crate::{
    cli::command::command::Far,
    usecases::{
        dry_run::dry_run_text,
        find::find_text,
        ignore_case::ignore_case,
        regex::find_regex,
        replace::{replace_text, replace_text_in_file},
    },
};
use std::{
    fs::{self, File},
    io::{Read, Write},
};

impl Far {
    pub fn control_args(self) {
        if self.backup.is_some()
            && (self.find.is_some() || self.replace.is_some() || self.dry_run || self.confirm)
        {
            eprintln!(
                "Err: --backup cannot be used with --find, --replace, --dry-run, or --confirm"
            );
            return;
        }

        if self.output.is_some() && (self.dry_run || self.confirm) {
            eprintln!("Err: --output cannot be used with --dry-run or --confirm");
            return;
        }

        if self.confirm || self.dry_run || self.ignore_case || self.output.is_some() {
            if self.find.is_none() && self.regex.is_none() {
                eprintln!(
                    "Err: --confirm, --dry-run, --ignore-case or --output requires either --find or --regex"
                );
                return;
            }
            if self.replace.is_none() {
                eprintln!("Err: --confirm, --dry-run or --output requires --replace");
                return;
            }

            if let Some(target) = &self.target {
                self.finding_options(target);
            } else {
                eprintln!(
                    "Err: --replace, --preview, --ignore-case, --output or --backup requires --target"
                );
            }
            return;
        } else if let Some(backup_file) = &self.backup {
            self.handle_file_backup(backup_file);
            return;
        }

        eprintln!(
            "Err: no valid operation selected. Use --find, --replace, --backup, --dry-run, --confirm, --ignore-case or --output"
        );
    }

    fn ignore_option(&self, path: &String, find: &String) {
        if (self.ignore_case && self.confirm)
            || (self.ignore_case && self.dry_run)
            || (self.ignore_case && self.output.is_some())
        {
            let result_word = ignore_case(find, path);
            self.actions(path, &result_word.to_lowercase(), true);
            return;
        } else {
            eprintln!("Err: Use --ignore-case with --confirm, --dry-run or --output");
        }
    }

    fn finding_options(&self, path: &String) {
        if let Some(find) = &self.find {
            self.ignore_option(path, find);

            let text_found = find_text(find, path);
            if text_found {
                self.actions(path, find, false);
            } else {
                eprintln!("Err: '{}' is not found in the given file", find);
            }
        } else if let Some(regex) = &self.regex {
            if let Some(regex_text) = find_regex(regex, path) {
                self.actions(path, &regex_text, false);
            } else {
                eprintln!("Err: '{}' is not found in the given file", regex);
            }
        }
    }

    fn actions(&self, taken_path: &String, find_text: &String, case: bool) {
        let replace_txt = self.replace.as_ref().unwrap();
        if self.confirm {
            replace_text(taken_path, find_text, replace_txt, case);
        } else if self.dry_run {
            self.handle_dry_run(replace_txt, case);
        } else if let Some(target_path) = &self.output {
            replace_text_in_file(taken_path, target_path, find_text, replace_txt, case);
        }
    }

    fn handle_dry_run(&self, replace_txt: &String, ignore_case: bool) {
        if self.target.is_some() {
            if let Some(target) = &self.target {
                if let Some(find) = &self.find {
                    dry_run_text(target, find, replace_txt, ignore_case);
                } else if let Some(regex) = &self.regex {
                    if let Some(regex_found) = find_regex(&regex, target) {
                        dry_run_text(target, &regex_found, replace_txt, ignore_case);
                    } else {
                        eprintln!("Err: '{}' is not found in the given file", regex);
                    }
                } else {
                    eprintln!("Err: use either --find or --regex flag to find the text");
                }
            } else {
                eprintln!("Err: failed to get the target");
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
            if let Some(target) = &self.target {
                let mut data_file =
                    File::create_new(&merge_path).expect("Err: failed to create the file");
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
            }
        } else {
            eprintln!("Err: {:?} file already exists", merge_path);
        }
    }
}
