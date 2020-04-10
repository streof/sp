use crate::searcher::{Searcher, SearchType};
use crate::writer::Writer;
use crate::matcher::MatcherBuilder;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

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

    #[structopt(
        name = "PATH",
        parse(from_os_str),
        help = "A file to search",
        long_help = "A file to search"
    )]
    pub path: PathBuf,

    /// Case insensitive search
    #[structopt(short, long)]
    pub ignore_case: bool,

    /// Limit number of shown matches
    #[structopt(short, long, value_name="NUM")]
    pub max_count: Option<u64>,

    /// Do not show line number which is enabled by default
    #[structopt(short, long)]
    pub no_line_number: bool,
}

pub type CliResult = anyhow::Result<(), anyhow::Error>;

impl Cli {
    pub fn show_matches(self, mut reader: impl BufRead, writer: impl Write) -> CliResult {

        let matcher = MatcherBuilder::new()
            .ignore_case(self.ignore_case)
            .max_count(self.max_count)
            .no_line_number(self.no_line_number)
            .build(self.pattern);

        let searcher = Searcher {
            reader: &mut reader,
            matcher: &matcher,
        };

        #[allow(clippy::match_bool)]
        let search_type = match self.max_count.is_some() {
            false => SearchType::Base(searcher),
            true => SearchType::MaxCount(searcher),
        };

        let matches = search_type.search_matches();

        let wrt = Writer { wrt: writer };
        wrt.print_matches(matches, &matcher.config)?;

        // Return () on success
        Ok(())
    }
}
