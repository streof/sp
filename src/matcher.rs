/// Internal configuration of our cli which can only by modified by MatcherBuilder.
#[derive(Clone, Debug)]
pub struct Config {
    pub ignore_case: bool,
    pub max_count: Option<u64>,
    pub no_line_number: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ignore_case: false,
            max_count: None,
            no_line_number: false,
        }
    }
}

pub struct Matcher {
    pub pattern: String,
    pub config: Config,
}

#[derive(Clone, Debug)]
pub struct MatcherBuilder {
    config: Config,
}

impl Default for MatcherBuilder {
    fn default() -> MatcherBuilder {
        MatcherBuilder::new()
    }
}

impl<'a> MatcherBuilder {
    /// Create a new Config builder with a default configuration.
    pub fn new() -> MatcherBuilder {
        MatcherBuilder {
            config: Config::default(),
        }
    }

    /// Disabled (i.e. false) by default
    pub fn ignore_case(&mut self, v: bool) -> &mut MatcherBuilder {
        self.config.ignore_case = v;
        self
    }

    /// Disabled (i.e. None) by default
    pub fn max_count(&mut self, v: Option<u64>) -> &mut MatcherBuilder {
        self.config.max_count = v;
        self
    }

    /// Disabled (i.e. false) by default
    pub fn no_line_number(&mut self, v: bool) -> &mut MatcherBuilder {
        self.config.no_line_number = v;
        self
    }

    /// Build MatcherBuilder
    pub fn build(&self, mut pattern: String) -> Matcher {
        if self.config.ignore_case {
            pattern = pattern.to_lowercase();
        }

        let config = Config {
            ignore_case: self.config.ignore_case,
            max_count: self.config.max_count,
            no_line_number: self.config.no_line_number,
        };

        Matcher { pattern, config }
    }
}
