use crate::{matcher::*, writer::*};
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

    /// Do not show line number which is enabled by default
    #[structopt(short, long)]
    pub no_line_number: bool,

    /// Limit number of shown matches
    #[structopt(short, long, value_name="NUM")]
    pub max_count: Option<u64>,
}

pub type CliResult = anyhow::Result<(), anyhow::Error>;

/// Internal configuration of our cli which can only by modified by ConfigBuilder.
#[derive(Clone, Debug)]
pub struct Config {
    pub no_line_number: bool,
    pub max_count: Option<u64>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            no_line_number: false,
            max_count: None,
        }
    }
}

/// Builder for our cli configurations; once built cheaper to reuse
#[derive(Clone, Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl Default for ConfigBuilder {
    fn default() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

impl ConfigBuilder {
    /// Create a new Config builder with a default configuration.
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            config: Config::default(),
        }
    }

    /// Disabled (i.e. false) by default
    pub fn no_line_number(&mut self, v: bool) -> &mut ConfigBuilder {
        self.config.no_line_number = v;
        self
    }

    /// Disabled (i.e. None) by default
    pub fn max_count(&mut self, v: Option<u64>) -> &mut ConfigBuilder {
        self.config.max_count = v;
        self
    }

    /// Build ConfigBuilder
    pub fn build(&self) -> Config {
        Config {
            no_line_number: self.config.no_line_number,
            max_count: self.config.max_count,
        }
    }
}

impl Cli {
    pub fn show_matches(&mut self, mut reader: impl BufRead, writer: impl Write) -> CliResult {
        let config = ConfigBuilder::new()
            .no_line_number(self.no_line_number)
            .max_count(self.max_count)
            .build();

        let mut matcher = Matcher {
            reader: &mut reader,
            pattern: &self.pattern,
            config: &config,
        };

        let wrt = Writer { wrt: writer };

        let matches = matcher.get_matches();
        wrt.print_matches(matches, &config)?;

        // Return () on success
        Ok(())
    }
}
