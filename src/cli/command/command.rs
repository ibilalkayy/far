use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "far stands for Find And Replace — a fast, flexible command-line tool to search and replace text across files and folders."
)]
pub struct Far {
    #[clap(short, long)]
    pub find: String,

    #[clap(short, long)]
    pub replace: String,

    #[clap(short, long)]
    pub target: Option<String>,

    #[clap(long)]
    pub dry_run: bool,

    #[clap(long)]
    pub confirm: bool,

    #[clap(long)]
    pub ignore_case: bool
}
