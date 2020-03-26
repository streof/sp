use crate::{matcher::*, writer::*};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    #[structopt(
        name = "PATTERN",
        help = "A pattern used for matching a sub-slice",
        long_help = "A pattern used for matching a sub-slice"
    )]
    pub pattern: String,

    #[structopt(
        name = "PATH",
        parse(from_os_str),
        help = "A file to search",
        long_help = "A file to search"
    )]
    pub path: PathBuf,

    /// Do not show line number which is enabled by default
    #[structopt(short, long)]
    pub no_line_number: bool,
}

pub type CliResult = anyhow::Result<(), anyhow::Error>;

impl Cli {
    pub fn show_matches(&mut self, mut reader: impl BufRead, writer: impl Write) -> CliResult {
        let mut matcher = Matcher {
            reader: &mut reader,
            pattern: &self.pattern,
        };

        let wrt = Writer { wrt: writer };

        let matches = matcher.get_matches();
        wrt.print_matches(matches)?;

        // Return () on success
        Ok(())
    }
}
