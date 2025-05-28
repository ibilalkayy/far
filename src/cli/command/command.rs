use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "far stands for Find And Replace — a fast, flexible command-line tool to search and replace text across files and folders."
)]
pub struct Far {
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
