use crate::matcher::{Matcher, MatcherType};
use bstr::BString;
use std::cmp::PartialEq;
use std::io::BufRead;

pub struct Searcher<'a, R> {
    pub reader: R,
    pub matcher: &'a Matcher,
}

pub struct SearchResult {
    pub matches: Vec<BString>,
    pub line_numbers: LineNumbers,
}

#[derive(Debug, PartialEq)]
pub enum LineNumbers {
    None,
    Some(Vec<u64>),
}

pub type SearcherResult = Result<SearchResult, std::io::Error>;

pub trait ReturnSearcherResult {
    fn ret_searcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> SearcherResult;
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

pub struct Init {
    pub matches: Vec<BString>,
    pub line_numbers_inner: Vec<u64>,
    pub line_number: u64,
}

impl Default for Init {
    fn default() -> Init {
        Init {
            matches: Default::default(),
            line_numbers_inner: Default::default(),
            line_number: Default::default(),
        }
    }
}

impl<'a, R: BufRead> Searcher<'a, R> {
    pub fn search_matches(mut self) -> SearcherResult {
        let matcher_type = &self.matcher.matcher_type;
        match matcher_type {
            MatcherType::Base => {
                use crate::base::BaseSearch;
                self.get_matches()
            }
            MatcherType::MaxCount => {
                use crate::max_count::MaxCountSearch;
                self.get_matches()
            }
        }
    }
}
