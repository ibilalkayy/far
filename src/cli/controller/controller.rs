use crate::{
    cli::command::command::Far,
    usecases::{dry_run::dry_run_text, find::find_text, replace::replace_text},
};
use clap::Parser;

pub fn control_args() {
    let args: Far = Far::parse();
    
    if args.dry_run {
        if let Some(path) = args.target.clone() {
            dry_run_text(path, args.find, args.replace);
        } else {
            eprintln!("Dry run needs a --target file path to preview.");
        }
    } else if let Some(path) = args.target.clone() {
        let text_found = find_text(args.find.clone(), path.clone());
        if text_found {
            if args.confirm {
                replace_text(path, args.find, args.replace);
            } else {
                eprintln!("Use --confirm to write the replace the text")
            }
        } else {
            eprintln!("Err: '{}' not found in the given file", args.find);
        }
    } else {
        eprintln!("Nothing to do. Use either --target to write or --dry-run to preview.");
    }
}
