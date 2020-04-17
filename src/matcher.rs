/// Internal configuration of our cli which can only by modified by MatcherBuilder.
#[derive(Clone, Debug)]
pub struct Config {
    pub ends_with: bool,
    pub ignore_case: bool,
    pub max_count: Option<u64>,
    pub no_line_number: bool,
    pub starts_with: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ends_with: false,
            ignore_case: false,
            max_count: None,
            no_line_number: false,
            starts_with: false,
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
        };

        #[allow(clippy::match_bool)]
        let matcher_type = match (
            self.config.ends_with,
            self.config.starts_with,
            self.config.max_count.is_some(),
        ) {
            // TODO: Fix this by implementing exact word matching
            (true, true, _) => MatcherType::StartsEndsWith,
            (true, false, _) => MatcherType::EndsWith,
            (false, true, _) => MatcherType::StartsWith,
            (false, false, true) => MatcherType::MaxCount,
            (false, false, false) => MatcherType::Base,
        };

        Matcher {
            pattern,
            config,
            matcher_type,
        }
    }
}
