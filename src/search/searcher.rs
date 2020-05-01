use crate::matcher::{Matcher, MatcherType};
use crate::results::GenResult;
use crate::search::{Base, EndsWith, MaxCount, StartsEndsWith, StartsWith, Words};
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
            MatcherType::Base => Base::get_matches(&mut self),
            MatcherType::EndsWith => EndsWith::get_matches(&mut self),
            MatcherType::MaxCount => MaxCount::get_matches(&mut self),
            MatcherType::StartsWith => StartsWith::get_matches(&mut self),
            MatcherType::StartsEndsWith => StartsEndsWith::get_matches(&mut self),
            MatcherType::Words => Words::get_matches(&mut self),
        }
    }
}
