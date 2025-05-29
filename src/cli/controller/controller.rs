use crate::{
    cli::command::command::Fara,
    usecases::{
        backup::file_backup, dry_run::dry_run_text, find::find_text, ignore_case::ignore_case,
        regex::find_regex, replace::replace_text,
    },
};

impl Fara {
    pub fn control_args(self) {
        if self.backup.is_some()
            && (self.find.is_some()
                || self.regex.is_some()
                || self.replace.is_some()
                || self.dry_run
                || self.confirm
                || self.output.is_some())
        {
            eprintln!("Err: --backup cannot be used with the given flags except --target");
            return;
        } else if self.backup.is_some() && self.target.is_none() {
            eprintln!("Err: --backup requires the --target flag");
            return;
        }

        if self.confirm && (self.dry_run || self.output.is_some() || self.ignore_case) {
            eprintln!("Err: --confirm cannot be used with --dry-run, --ignore-case or --output");
            return;
        }

        if self.output.is_some() && self.dry_run {
            eprintln!("Err: --output cannot be used with --dry-run");
            return;
        }

        if self.output.is_some() && self.ignore_case {
            eprintln!("Err: --output cannot be used with --ignore-case");
            return;
        }

        if self.confirm || self.dry_run || self.output.is_some() {
            if self.find.is_none() && self.regex.is_none() {
                eprintln!(
                    "Err: --confirm, --dry-run or --output requires either --find or --regex"
                );
                return;
            }
            if self.replace.is_none() {
                eprintln!("Err: --confirm, --dry-run or --output requires --replace");
                return;
            }

            if self.target.is_none() {
                eprintln!("Err: --confirm, --dry-run or --output requires --target");
                return;
            }

            if let Some(target) = &self.target {
                self.finding_options(target);
                return;
            } else {
                eprintln!("Err: --replace, --dry-run, --output or --backup requires --target");
                return;
            }
        } else {
            if self.backup.is_some() {
                if let Some(backup_file) = &self.backup {
                    if let Some(target) = &self.target {
                        file_backup(backup_file, target);
                        return;
                    }
                }
            } else if self.ignore_case {
                if self.find.is_none() && self.regex.is_none() {
                    eprintln!("Err: --ignore-case requires either --find or --regex");
                    return;
                }
                if self.replace.is_none() {
                    eprintln!("Err: --ignore-case requires --replace");
                    return;
                }

                if self.target.is_none() {
                    eprintln!("Err: --ignore-case requires --target");
                    return;
                }

                if let Some(find) = &self.find {
                    if let Some(target) = &self.target {
                        if let Some(replace) = &self.replace {
                            let data = ignore_case(find, target);
                            replace_text(target, target, &data, replace);
                            return;
                        }
                    }
                }
            }
        }
        eprintln!("Err: no valid operation selected. Write 'fara --help' for more info");
    }

    fn finding_options(&self, path: &String) {
        if let Some(find) = &self.find {
            let text_found = find_text(find, path);
            if text_found {
                self.actions(path, find);
            } else {
                eprintln!("Err: '{}' text is not found in a given file", find);
            }
        } else if let Some(regex) = &self.regex {
            if let Some(regex_text) = find_regex(regex, path) {
                self.actions(path, &regex_text);
            } else {
                eprintln!(
                    "Err: text with this expression '{}' is not found in a given file",
                    regex
                );
            }
        }
    }

    fn actions(&self, inner_path: &String, find_text: &String) {
        let replace_txt = self.replace.as_ref().unwrap();
        if self.confirm {
            replace_text(inner_path, inner_path, find_text, replace_txt);
        } else if self.dry_run {
            self.handle_dry_run(replace_txt);
        } else if let Some(outer_path) = &self.output {
            replace_text(inner_path, outer_path, find_text, replace_txt);
        }
    }

    fn handle_dry_run(&self, replace_txt: &String) {
        if self.target.is_some() {
            if let Some(target) = &self.target {
                if let Some(find) = &self.find {
                    dry_run_text(target, find, replace_txt);
                } else if let Some(regex) = &self.regex {
                    if let Some(regex_found) = find_regex(&regex, target) {
                        dry_run_text(target, &regex_found, replace_txt);
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
}
