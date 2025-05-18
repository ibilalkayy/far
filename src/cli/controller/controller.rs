use crate::{
    cli::command::command::Far,
    usecases::{dry_run::dry_run_text, find::find_text, regex::find_regex, replace::replace_text},
};
use clap::Parser;

pub fn control_args() {
    let args: Far = Far::parse();

    if args.dry_run {
        if let Some(path) = args.target.clone() {
            if let Some(find) = args.find {
                dry_run_text(path, find, args.replace);
            } else {
                eprintln!("Err: value is not provided to find the text");
            }
        } else {
            eprintln!("Dry run needs a --target file path to preview.");
        }
    } else if let Some(path) = args.target.clone() {
        if let Some(find) = args.find.clone() {
            let text_found = find_text(find.clone(), path.clone());
            if text_found {
                if args.confirm {
                    replace_text(path, find, args.replace);
                } else {
                    eprintln!("Use --confirm to replace the text")
                }
            } else {
                eprintln!("Err: '{}' is not found in the given file", find.clone());
            }
        } else if let Some(regex) = args.regex.clone() {
            if let Some(result) = find_regex(regex.clone(), path.clone()) {
                if args.confirm {
                    println!("Regex text: {}", result);
                    replace_text(path, result, args.replace);
                } else {
                    eprintln!("Use --confirm to replace the text");
                }
            } else {
                eprintln!("Err: '{}' is not found in the given file", regex.clone());
            }
        } else {
            eprintln!("Err: value is not provided to find the text");
        }
    } else {
        eprintln!("Nothing to do. Use either --target to write or --dry-run to preview.");
    }
}
