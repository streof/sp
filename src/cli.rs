use crate::searcher::Searcher;
use crate::writer::Writer;
use crate::matcher::MatcherBuilder;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use structopt::{StructOpt, clap::AppSettings};

const ABOUT: &str = "
grrs is a very basic implementation of grep. Use -h for more information.";

const USAGE: &str = "
    grrs [OPTIONS] <PATTERN> <PATH>";

const TEMPLATE: &str = "\
{bin} {version}
{about}

USAGE:{usage}

ARGS:
{positionals}

OPTIONS:
{unified}";

// AppSettings::DeriveDisplayOrder might be helpful for custom ordering
// AppSettings::HidePossibleValuesInHelp for concise usage message
#[structopt(rename_all = "kebab-case", about = ABOUT, usage = USAGE, 
    template = TEMPLATE, 
    global_settings(&[AppSettings::UnifiedHelpMessage]))]
#[derive(StructOpt)]
pub struct Cli {
    #[structopt(
        name = "PATTERN",
        help = "A pattern used for matching a sub-slice",
        long_help = "A pattern used for matching a sub-slice"
    )]
    pub pattern: String,
    // TODO: pattern should be optional if -c is provided

    #[structopt(
        name = "PATH",
        parse(from_os_str),
        help = "A file to search",
        long_help = "A file to search"
    )]
    pub path: PathBuf,

    /// Suppress normal output and show number of matching lines
    #[structopt(short, long)]
    pub count: bool,

    /// Only show matches containing fields ending with PATTERN
    #[structopt(short, long)]
    pub ends_with: bool,

    /// Case insensitive search
    #[structopt(short, long)]
    pub ignore_case: bool,

    /// Limit number of shown matches
    #[structopt(short, long, value_name="NUM")]
    pub max_count: Option<u64>,

    /// Suppress line numbers which are shown by default
    #[structopt(short, long)]
    pub no_line_number: bool,

    /// Only show matches containing fields starting with PATTERN
    #[structopt(short, long)]
    pub starts_with: bool,

    /// Whole words search (i.e. non-word characters are stripped)
    ///
    /// This flag overrides --starts-with and --ends-with and is
    /// roughly equivalent to putting \b before and after PATTERN
    #[structopt(short, long)]
    pub words: bool,
}

pub type CliResult = anyhow::Result<(), anyhow::Error>;

impl Cli {
    pub fn show_matches(self, mut reader: impl BufRead, writer: impl Write) -> CliResult {

        let matcher = MatcherBuilder::new()
            .count(self.count)
            .ends_with(self.ends_with)
            .ignore_case(self.ignore_case)
            .max_count(self.max_count)
            .no_line_number(self.no_line_number)
            .starts_with(self.starts_with)
            .words(self.words)
            .build(self.pattern);

        let searcher = Searcher {
            reader: &mut reader,
            matcher: &matcher,
        };

        let wrt = Writer { wrt: writer };
        let matches = searcher.search_matches();

        wrt.print_matches(matches, &matcher.config)?;

        // Return () on success
        Ok(())
    }
}
