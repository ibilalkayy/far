mod cli;
mod usecases;

use clap::Parser;
use cli::command::command::Fara;

fn main() {
    let args = Fara::parse();
    args.control_args();
}
// Hello world this is it
