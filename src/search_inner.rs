use crate::ext::ByteSliceExt;
use bstr::{BString, ByteSlice};

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
    line.trim_terminator()
        .split_str(" ")
        .any(|word| word.starts_with_str(pattern))
}

pub fn check_contains(line: &[u8], pattern: &str) -> bool {
    line.contains_str(pattern)
}
