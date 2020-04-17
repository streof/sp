/// Internal configuration of our cli which can only by modified by MatcherBuilder.
#[derive(Clone, Debug)]
pub struct Config {
    pub ends_with: bool,
    pub ignore_case: bool,
    pub max_count: Option<u64>,
    pub no_line_number: bool,
    pub starts_with: bool,
    pub words: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ends_with: false,
            ignore_case: false,
            max_count: None,
            no_line_number: false,
            starts_with: false,
            words: false,
        }
    }
}

pub struct Matcher {
    pub pattern: String,
    pub config: Config,
    pub matcher_type: MatcherType,
}

pub enum MatcherType {
    Base,
    EndsWith,
    MaxCount,
    StartsEndsWith,
    StartsWith,
    Words,
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
    pub fn ends_with(&mut self, v: bool) -> &mut MatcherBuilder {
        self.config.ends_with = v;
        self
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

    /// Disabled (i.e. false) by default
    pub fn starts_with(&mut self, v: bool) -> &mut MatcherBuilder {
        self.config.starts_with = v;
        self
    }

    /// Disabled (i.e. false) by default
    pub fn words(&mut self, v: bool) -> &mut MatcherBuilder {
        self.config.words = v;
        self
    }

    /// Build MatcherBuilder
    pub fn build(&self, mut pattern: String) -> Matcher {
        if self.config.ignore_case {
            pattern = pattern.to_lowercase();
        }

        let config = Config {
            ends_with: self.config.ends_with,
            ignore_case: self.config.ignore_case,
            max_count: self.config.max_count,
            no_line_number: self.config.no_line_number,
            starts_with: self.config.starts_with,
            words: self.config.words,
        };

        let matcher_type = match (
            self.config.words,
            self.config.ends_with,
            self.config.starts_with,
            self.config.max_count.is_some(),
        ) {
            (true, _, _, _) => MatcherType::Words,
            (false, true, true, _) => MatcherType::StartsEndsWith,
            (false, true, false, _) => MatcherType::EndsWith,
            (false, false, true, _) => MatcherType::StartsWith,
            (false, false, false, true) => MatcherType::MaxCount,
            (false, false, false, false) => MatcherType::Base,
        };

        Matcher {
            pattern,
            config,
            matcher_type,
        }
    }
}
