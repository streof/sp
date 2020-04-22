use crate::ext::ByteSliceExt;
use bstr::{BString, ByteSlice};

#[derive(Debug, PartialEq)]
pub enum LineNumbers {
    None,
    Some(Vec<u64>),
}

pub struct SearchResult {
    pub matches: Vec<BString>,
    pub line_numbers: LineNumbers,
}

pub enum GenInnerResult {
    Search(SearchResult),
    Count(CountResult),
}

pub type SearcherResult = Result<SearchResult, std::io::Error>;
pub type CounterResult = Result<CountResult, std::io::Error>;
pub type GenResult = Result<GenInnerResult, std::io::Error>;

#[derive(Debug)]
pub struct SearchInnerResult {
    pub matches: Vec<BString>,
    pub line_numbers: Vec<u64>,
}

impl Default for SearchInnerResult {
    fn default() -> SearchInnerResult {
        SearchInnerResult {
            matches: Default::default(),
            line_numbers: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct CountResult {
    pub count: u64,
}

impl Default for CountResult {
    fn default() -> CountResult {
        CountResult {
            count: Default::default(),
        }
    }
}

pub trait Upcast {
    type Type;

    fn upcast(self) -> Self::Type;
}

impl Upcast for SearchInnerResult {
    type Type = GenResult;
    fn upcast(self) -> Self::Type {
        let match_result = if self.line_numbers.is_empty() {
            SearchResult {
                matches: self.matches,
                line_numbers: LineNumbers::None,
            }
        } else {
            SearchResult {
                matches: self.matches,
                line_numbers: LineNumbers::Some(self.line_numbers),
            }
        };

        Ok(GenInnerResult::Search(match_result))
    }
}

impl Upcast for CountResult {
    type Type = GenResult;
    fn upcast(self) -> Self::Type {
        Ok(GenInnerResult::Count(self))
    }
}

impl CountResult {
    pub fn check_and_add<F>(&mut self, pattern: &str, line: &[u8], check: F)
    where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line, pattern) {
            self.count += 1;
        }
    }
}

impl SearchInnerResult {
    pub fn check_and_store<F>(&mut self, pattern: &str, line_number: u64, line: &[u8], check: F)
    where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line, pattern) {
            self.matches.push(line.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_nln<F>(&mut self, pattern: &str, line: &[u8], check: F)
    where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line, pattern) {
            self.matches.push(line.trim_terminator());
        }
    }

    pub fn check_and_store_nln_max_count<F>(
        &mut self,
        pattern: &str,
        line: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line, pattern) {
            *matches_left -= 1;
            self.matches.push(line.trim_terminator());
        }
    }

    pub fn check_and_store_separate<F>(
        &mut self,
        pattern: &str,
        line_number: u64,
        line_check: &[u8],
        line_store: &[u8],
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line_check, pattern) {
            self.matches.push(line_store.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_separate_nln<F>(
        &mut self,
        pattern: &str,
        line_check: &[u8],
        line_store: &[u8],
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line_check, pattern) {
            self.matches.push(line_store.trim_terminator());
        }
    }

    pub fn check_and_store_separate_nln_max_count<F>(
        &mut self,
        pattern: &str,
        line_check: &[u8],
        line_store: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line_check, pattern) {
            *matches_left -= 1;
            self.matches.push(line_store.trim_terminator());
        }
    }

    pub fn check_and_store_max_count<F>(
        &mut self,
        pattern: &str,
        line_number: u64,
        line: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line, pattern) {
            *matches_left -= 1;
            self.matches.push(line.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_separate_max_count<F>(
        &mut self,
        pattern: &str,
        line_number: u64,
        line_check: &[u8],
        line_store: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &str) -> bool,
    {
        if check(line_check, pattern) {
            *matches_left -= 1;
            self.matches.push(line_store.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }
}

pub fn check_starts_with(line: &[u8], pattern: &str) -> bool {
    line.fields().any(|word| word.starts_with_str(pattern))
}

pub fn check_ends_with(line: &[u8], pattern: &str) -> bool {
    line.fields().any(|word| word.ends_with_str(pattern))
}

pub fn check_starts_ends_with(line: &[u8], pattern: &str) -> bool {
    line.fields()
        .any(|word| word.starts_with_str(pattern) && word.ends_with_str(pattern))
}

// TODO: Probably redundant
pub fn check_contains(line: &[u8], pattern: &str) -> bool {
    line.contains_str(pattern)
}

pub fn check_words(line: &[u8], pattern: &str) -> bool {
    line.trim_terminator().words().any(|word| word == pattern)
}
