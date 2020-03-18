use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(name = "PATTERN", help = "A pattern used for matching a sub-slice")]
    pub pattern: String,

    #[structopt(name = "PATH", parse(from_os_str), help = "A file to search")]
    pub path: PathBuf,
}
