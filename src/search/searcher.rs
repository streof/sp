use crate::matcher::{Matcher, MatcherType};
use crate::results::GenResult;
use std::io::BufRead;

#[derive(Debug)]
pub struct Searcher<'a, R> {
    pub reader: R,
    pub matcher: &'a Matcher,
}

impl<'a, R: BufRead> Searcher<'a, R> {
    pub fn search_matches(mut self) -> GenResult {
        let matcher_type = &self.matcher.matcher_type;
        match matcher_type {
            MatcherType::Base => {
                use crate::search::Base;
                self.get_matches()
            }
            MatcherType::EndsWith => {
                use crate::search::EndsWith;
                self.get_matches()
            }
            MatcherType::MaxCount => {
                use crate::search::MaxCount;
                self.get_matches()
            }
            MatcherType::StartsWith => {
                use crate::search::StartsWith;
                self.get_matches()
            }
            MatcherType::StartsEndsWith => {
                use crate::search::StartsEndsWith;
                self.get_matches()
            }
            MatcherType::Words => {
                use crate::search::Words;
                self.get_matches()
            }
        }
    }
}
