use crate::matcher::{Matcher, MatcherType};
use bstr::BString;
use std::cmp::PartialEq;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum LineNumbers {
    None,
    Some(Vec<u64>),
}

pub struct SearchResult {
    pub matches: Vec<BString>,
    pub line_numbers: LineNumbers,
}

pub type SearcherResult = Result<SearchResult, std::io::Error>;

pub trait ReturnSearcherResult {
    fn ret_searcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> SearcherResult;
}

pub struct Searcher<'a, R> {
    pub reader: R,
    pub matcher: &'a Matcher,
}

impl<'a, R: BufRead> Searcher<'a, R> {
    pub fn search_matches(mut self) -> SearcherResult {
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
        }
    }
}

impl<'a, R: BufRead> ReturnSearcherResult for Searcher<'a, R> {
    /// Convenient method for wrapping `Vec<BString>` and `LineNumbers` before
    /// returning as `SearcherResult`
    fn ret_searcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> SearcherResult {
        let match_result = SearchResult {
            matches,
            line_numbers,
        };
        Ok(match_result)
    }
}
