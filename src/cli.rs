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

impl Cli {
    pub fn show_matches(
        &mut self,
        mut reader: impl BufRead,
        writer: impl Write,
    ) -> std::result::Result<(), anyhow::Error> {
        let mut matcher = Matcher {
            reader: &mut reader,
            pattern: &self.pattern,
        };
        let matches = matcher.get_matches();
        print_matches(writer, matches)?;

        // Return () on success
        Ok(())
    }
}