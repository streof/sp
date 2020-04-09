use crate::cli::Config;
use bstr::BString;
use std::cmp::PartialEq;
use std::io::BufRead;

pub struct Matcher<'a, R> {
    pub reader: R,
    pub pattern: &'a str,
    pub config: &'a Config,
}

pub struct MatchResult {
    pub matches: Vec<BString>,
    pub line_numbers: LineNumbers,
}

#[derive(Debug, PartialEq)]
pub enum LineNumbers {
    None,
    Some(Vec<u64>),
}

pub type MatcherResult = Result<MatchResult, std::io::Error>;

pub trait ReturnMatcherResult {
    fn ret_matcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> MatcherResult;
}

impl<'a, R: BufRead> ReturnMatcherResult for Matcher<'a, R> {
    /// Convenient method for wrapping `Vec<BString>` and `LineNumbers` before
    /// returning as `MatcherResult`
    fn ret_matcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> MatcherResult {
        let match_result = MatchResult {
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

pub enum MatcherType<'a, R> {
    Base(Matcher<'a, R>),
    MaxCount(Matcher<'a, R>),
}

impl<'a, R: BufRead> MatcherType<'a, R> {
    pub fn find_matches(self) -> MatcherResult {
        match self {
            MatcherType::Base(mut m) => {
                use crate::base::BaseMatches;
                Matcher::get_matches(&mut m)
            }
            MatcherType::MaxCount(mut m) => {
                use crate::max_count::MaxCountMatches;
                Matcher::get_matches(&mut m)
            }
        }
    }
}
