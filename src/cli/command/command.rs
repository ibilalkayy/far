use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "fara stands for Find And Replace Application — a fast, flexible command-line tool to search and replace text across files and folders."
)]
pub struct Fara {
    /// Find the text to be changed
    #[clap(short, long)]
    pub find: Option<String>,

    /// Use expressions for finding the text
    #[clap(short = 'e', long)]
    pub regex: Option<String>,

    /// Write the text to be replaced with
    #[clap(short, long)]
    pub replace: Option<String>,

    /// Mention the target path to find the text there
    #[clap(short, long)]
    pub target: Option<String>,

    /// Include the files that are matching the globe pattern
    #[clap(long)]
    pub backup: Option<String>,

    /// Ignore the case while finding the data
    #[clap(long)]
    pub ignore_case: bool,

    /// Assure the text before replacing it
    #[clap(long)]
    pub confirm: bool,

    /// Show the replaced text before writing it
    #[clap(long)]
    pub dry_run: bool,

    /// Store the modified text into another file
    #[clap(long)]
    pub output: Option<String>,
}
