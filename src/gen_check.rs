use crate::search_inner::*;
use crate::searcher::*;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

// Explicit lifetime annotation is required as it has to match the annotation
// used when defining the check functions in search_inner (which was in this
// case omitted and hence inferred)
pub trait GenCheck {
    fn no_line_number<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn no_line_number_caseless<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn no_line_number_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn no_line_number_caseless_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn line_number<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn line_number_caseless<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn line_number_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
    fn line_number_caseless_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult;
}

impl<'a, R: BufRead> GenCheck for Searcher<'a, R> {
    fn no_line_number<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if line.contains_str(pattern) {
                sir.check_and_store_nln(pattern, line, &check);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::None)
    }

    fn no_line_number_caseless<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if line.is_ascii() {
                let line_lower = line.to_ascii_lowercase();
                if line_lower.contains_str(pattern) {
                    sir.check_and_store_separate_nln(pattern, &line_lower, line, &check);
                }
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    sir.check_and_store_separate_nln(pattern, &buf, line, &check);
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::None)
    }

    fn no_line_number_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            if line.contains_str(pattern) {
                sir.check_and_store_nln_max_count(pattern, line, &mut matches_left, &check);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::None)
    }

    fn no_line_number_caseless_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            if line.is_ascii() {
                let line_lower = line.to_ascii_lowercase();
                if line_lower.contains_str(pattern) {
                    sir.check_and_store_separate_nln_max_count(
                        pattern,
                        &line_lower,
                        line,
                        &mut matches_left,
                        &check,
                    );
                }
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    sir.check_and_store_separate_nln_max_count(
                        pattern,
                        &buf,
                        line,
                        &mut matches_left,
                        &check,
                    );
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::None)
    }

    fn line_number<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;

            if line.contains_str(pattern) {
                sir.check_and_store(pattern, line_number, line, &check);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_caseless<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            if line.is_ascii() {
                let line_lower = line.to_ascii_lowercase();
                if line_lower.contains_str(pattern) {
                    sir.check_and_store_separate(pattern, line_number, &line_lower, line, &check);
                }
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    sir.check_and_store_separate(pattern, line_number, &buf, line, &check);
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            line_number += 1;
            if line.contains_str(pattern) {
                sir.check_and_store_max_count(
                    pattern,
                    line_number,
                    line,
                    &mut matches_left,
                    &check,
                );
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_caseless_max_count<F: for<'r, 's> Fn(&'r [u8], &'s str) -> bool>(
        &mut self,
        check: F,
    ) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            line_number += 1;
            if line.is_ascii() {
                let line_lower = line.to_ascii_lowercase();
                if line_lower.contains_str(pattern) {
                    sir.check_and_store_separate_max_count(
                        pattern,
                        line_number,
                        &line_lower,
                        line,
                        &mut matches_left,
                        &check,
                    );
                }
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    sir.check_and_store_separate_max_count(
                        pattern,
                        line_number,
                        &buf,
                        line,
                        &mut matches_left,
                        &check,
                    );
                }
            }
            Ok(true)
        })?;
        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }
}
