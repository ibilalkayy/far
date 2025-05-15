use crate::{
    cli::command::command::Far,
    usecases::{find::find_text, replace::replace_text},
};
use clap::Parser;

pub fn control_args() {
    let args: Far = Far::parse();
    let status = find_text(args.find.clone(), args.target.clone());
    if status {
        replace_text(args.target, args.find, args.replace);
    } else {
        eprintln!("Err: '{}' text is not found in the given target path", args.find);
    }
}
