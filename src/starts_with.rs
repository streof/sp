use crate::searcher::*;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

trait StartsWith {
    // fn no_line_number(&mut self) -> SearcherResult;
    // fn no_line_number_caseless(&mut self) -> SearcherResult;
    // fn no_line_number_max_count(&mut self) -> SearcherResult;
    // fn no_line_number_caseless_max_count(&mut self) -> SearcherResult;
    fn line_number(&mut self) -> SearcherResult;
    fn line_number_caseless(&mut self) -> SearcherResult;
    fn line_number_max_count(&mut self) -> SearcherResult;
    fn line_number_caseless_max_count(&mut self) -> SearcherResult;
}

pub trait StartsWithSearch {
    fn get_matches(&mut self) -> SearcherResult;
}

impl<'a, R: BufRead> StartsWith for Searcher<'a, R> {
    fn line_number(&mut self) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;

            if line.contains_str(pattern) {
                sir.check_and_store(pattern, line_number, line);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_caseless(&mut self) -> SearcherResult {
        let (reader, pattern) = (&mut self.reader, &self.matcher.pattern);

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            if line.is_ascii() {
                let line_lower = line.to_ascii_lowercase();
                if line_lower.contains_str(pattern) {
                    sir.check_and_store_separate(pattern, line_number, &line_lower, line);
                }
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    sir.check_and_store_separate(pattern, line_number, &buf, line);
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_max_count(&mut self) -> SearcherResult {
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
                sir.check_and_store_max_count(pattern, line_number, line, &mut matches_left);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }

    fn line_number_caseless_max_count(&mut self) -> SearcherResult {
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
                    );
                }
            }
            Ok(true)
        })?;
        Self::ret_searcher_result(sir.matches, LineNumbers::Some(sir.line_numbers))
    }
}

impl<'a, R: BufRead> StartsWithSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> SearcherResult {
        let (ignore_case, max_count, no_line_number) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
        );

        match (no_line_number, ignore_case, max_count) {
            // (true, true, Some(_)) => self.no_line_number_caseless_max_count(),
            // (true, true, None) => self.no_line_number_caseless(),
            // (true, false, Some(_)) => self.no_line_number_max_count(),
            // (true, false, None) => self.no_line_number(),
            (false, true, Some(_)) => self.line_number_caseless_max_count(),
            (false, true, None) => self.line_number_caseless(),
            (false, false, Some(_)) => self.line_number_max_count(),
            (false, false, None) => self.line_number(),
            // TODO: remove this line after implementing -n APIs
            (_, _, _) => self.line_number(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matcher::MatcherBuilder;
    use std::io::Cursor;

    const LINE: &str = "again\ngain\na\x00nd, gain\n& AΓain\ngain";

    #[test]
    fn line_number() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(false)
            .max_count(Some(2))
            .no_line_number(false)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![2, 3];

        assert!(matches.len() == 2);
        assert_eq!(matches[0], &b"gain"[..]);
        assert_eq!(matches[1], &b"a\x00nd, gain"[..]);
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "aγain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(false)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![4];

        assert!(matches.len() == 1);
        assert_eq!(matches[0], "& AΓain".as_bytes());
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }
}
