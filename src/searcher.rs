use crate::matcher::Matcher;
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
            matches: vec![],
            line_numbers_inner: vec![],
            line_number: 0,
        }
    }
}

pub enum SearchType<'a, R> {
    Base(Searcher<'a, R>),
    MaxCount(Searcher<'a, R>),
}

impl<'a, R: BufRead> SearchType<'a, R> {
    pub fn search_matches(self) -> SearcherResult {
        match self {
            SearchType::Base(mut m) => {
                use crate::base::BaseSearch;
                Searcher::get_matches(&mut m)
            }
            SearchType::MaxCount(mut m) => {
                use crate::max_count::MaxCountSearch;
                Searcher::get_matches(&mut m)
            }
        }
    }
}
