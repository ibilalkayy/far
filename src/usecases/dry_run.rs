use std::fs;

use regex::RegexBuilder;

pub fn dry_run_text(taken_path: &str, from: &str, to: &str, ignore_case: bool) {
    let content = fs::read_to_string(taken_path).unwrap();

    let file_content = if ignore_case {
        let replaced = RegexBuilder::new(from)
            .case_insensitive(true)
            .build()
            .unwrap();
        replaced.replace_all(&content, to).to_string()
    } else {
        content.replace(from, to)
    };

    println!("{}", file_content);
}
