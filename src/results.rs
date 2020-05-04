use crate::ext::ByteSliceExt;
use bstr::{BString, ByteSlice};
use std::str;

#[derive(Debug, PartialEq)]
pub enum LineNumbers {
    None,
    Some(Vec<u64>),
}

impl Default for LineNumbers {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SearchResult {
    pub matches: Vec<BString>,
    pub line_numbers: LineNumbers,
}

#[derive(Debug, PartialEq)]
pub enum GenInnerResult {
    Search(SearchResult),
    Count(CountResult),
}

pub type GenResult = Result<GenInnerResult, std::io::Error>;

#[derive(Debug, Default)]
pub struct SearchInnerResult {
    pub matches: Vec<BString>,
    pub line_numbers: Vec<u64>,
}

#[derive(Debug, Default, PartialEq)]
pub struct CountResult {
    pub count: u64,
}

pub trait Upcast {
    /// Note that even though this returns a `GenResult`, it cannot fail
    fn upcast(self) -> GenResult;
}

impl Upcast for SearchInnerResult {
    fn upcast(self) -> GenResult {
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
    fn upcast(self) -> GenResult {
        Ok(GenInnerResult::Count(self))
    }
}

impl CountResult {
    pub fn check_and_add<F>(&mut self, pattern: &[u8], line: &[u8], check: F)
    where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line, pattern) {
            self.count += 1;
        }
    }
}

impl SearchInnerResult {
    pub fn check_and_store<F>(&mut self, pattern: &[u8], line_number: u64, line: &[u8], check: F)
    where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line, pattern) {
            self.matches.push(line.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_nln<F>(&mut self, pattern: &[u8], line: &[u8], check: F)
    where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line, pattern) {
            self.matches.push(line.trim_terminator());
        }
    }

    pub fn check_and_store_nln_max_count<F>(
        &mut self,
        pattern: &[u8],
        line: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line, pattern) {
            *matches_left -= 1;
            self.matches.push(line.trim_terminator());
        }
    }

    pub fn check_and_store_separate<F>(
        &mut self,
        pattern: &[u8],
        line_number: u64,
        line_check: &[u8],
        line_store: &[u8],
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line_check, pattern) {
            self.matches.push(line_store.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_separate_nln<F>(
        &mut self,
        pattern: &[u8],
        line_check: &[u8],
        line_store: &[u8],
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line_check, pattern) {
            self.matches.push(line_store.trim_terminator());
        }
    }

    pub fn check_and_store_separate_nln_max_count<F>(
        &mut self,
        pattern: &[u8],
        line_check: &[u8],
        line_store: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line_check, pattern) {
            *matches_left -= 1;
            self.matches.push(line_store.trim_terminator());
        }
    }

    pub fn check_and_store_max_count<F>(
        &mut self,
        pattern: &[u8],
        line_number: u64,
        line: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line, pattern) {
            *matches_left -= 1;
            self.matches.push(line.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }

    pub fn check_and_store_separate_max_count<F>(
        &mut self,
        pattern: &[u8],
        line_number: u64,
        line_check: &[u8],
        line_store: &[u8],
        matches_left: &mut u64,
        check: F,
    ) where
        F: Fn(&[u8], &[u8]) -> bool,
    {
        if check(line_check, pattern) {
            *matches_left -= 1;
            self.matches.push(line_store.trim_terminator());
            self.line_numbers.push(line_number);
        }
    }
}

pub fn check_starts_with(line: &[u8], pattern: &[u8]) -> bool {
    line.fields().any(|word| word.starts_with_str(pattern))
}

pub fn check_ends_with(line: &[u8], pattern: &[u8]) -> bool {
    line.fields().any(|word| word.ends_with_str(pattern))
}

pub fn check_starts_ends_with(line: &[u8], pattern: &[u8]) -> bool {
    line.fields()
        .any(|word| word.starts_with_str(pattern) && word.ends_with_str(pattern))
}

/// Only used by `base` and `max_count` modules
pub fn check_contains(line: &[u8], pattern: &[u8]) -> bool {
    line.contains_str(pattern)
}

// TODO: Would be nice (also for performance reasons in some cases, although very
// minimal) if pattern here was `&str` which would prevent the UTF-8 conversion
// inside the closure. One way to achieve that would be to make `GenSearch`'s
// functions accept `AsRef<[u8]>` and then in each function check if `--words` has
// been specified. If that is the case, leave pattern as `&str` and else convert
// to `&[u8]`.
// Unfortunately, this does not work as it's not possible to have something like
// nested trait bounds. Currently the only way to achieve it in our design would
// be to replicate the `GenSearch` trait and replace `&[u8]` by `&str`
pub fn check_words(line: &[u8], pattern: &[u8]) -> bool {
    let pattern_utf8 =
        str::from_utf8(pattern).expect("Should never panic: pattern is always UTF-8");
    line.trim_terminator()
        .words()
        .any(|word| word == pattern_utf8)
}
