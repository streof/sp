use crate::matcher::{Matcher, MatcherType};
use crate::results::GenResult;
use std::io::BufRead;

pub struct Searcher<'a, R> {
    pub reader: R,
    pub matcher: &'a Matcher,
}

impl<'a, R: BufRead> Searcher<'a, R> {
    pub fn search_matches(mut self) -> GenResult {
        let matcher_type = &self.matcher.matcher_type;
        match matcher_type {
            MatcherType::Base => {
                use crate::base::BaseSearch;
                self.get_matches()
            }
            MatcherType::EndsWith => {
                use crate::ends_with::EndsWithSearch;
                self.get_matches()
            }
            MatcherType::MaxCount => {
                use crate::max_count::MaxCountSearch;
                self.get_matches()
            }
            MatcherType::StartsWith => {
                use crate::starts_with::StartsWithSearch;
                self.get_matches()
            }
            MatcherType::StartsEndsWith => {
                use crate::starts_ends_with::StartsEndsWithSearch;
                self.get_matches()
            }
            MatcherType::Words => {
                use crate::words::WordsSearch;
                self.get_matches()
            }
        }
    }
}
