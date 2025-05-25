mod cli;
mod usecases;

use clap::Parser;
use cli::command::command::Far;

fn main() {
    let args = Far::parse();
    args.control_args();
}
// hello world this is it
