use crate::ext::ByteSliceExt;
use crate::searcher::*;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

trait MaxCount {
    fn no_line_number(&mut self) -> SearcherResult;
    fn no_line_number_caseless(&mut self) -> SearcherResult;
    fn line_number(&mut self) -> SearcherResult;
    fn line_number_caseless(&mut self) -> SearcherResult;
}

pub trait MaxCountSearch {
    fn get_matches(&mut self) -> SearcherResult;
}

impl<'a, R: BufRead> MaxCount for Searcher<'a, R> {
    fn no_line_number(&mut self) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let Init { mut matches, .. } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            if line.contains_str(pattern) {
                matches_left -= 1;
                matches.push(line.trim_terminator());
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(matches, LineNumbers::None)
    }

    fn no_line_number_caseless(&mut self) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let Init { mut matches, .. } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            if line.is_ascii() {
                if line.to_ascii_lowercase().contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                }
            } else {
                let mut buf = vec![];
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(matches, LineNumbers::None)
    }

    fn line_number(&mut self) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let Init {
            mut matches,
            mut line_numbers_inner,
            mut line_number,
        } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            line_number += 1;
            if line.contains_str(pattern) {
                matches_left -= 1;
                matches.push(line.trim_terminator());
                line_numbers_inner.push(line_number);
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }

    fn line_number_caseless(&mut self) -> SearcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.matcher.pattern,
            self.matcher.config.max_count.unwrap(),
        );

        let Init {
            mut matches,
            mut line_numbers_inner,
            mut line_number,
        } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            line_number += 1;
            if line.is_ascii() {
                if line.to_ascii_lowercase().contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            } else {
                let mut buf = vec![];
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            }
            Ok(true)
        })?;

        Self::ret_searcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }
}

impl<'a, R: BufRead> MaxCountSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> SearcherResult {
        let (ignore_case, no_line_number) = (
            self.matcher.config.ignore_case,
            self.matcher.config.no_line_number,
        );

        match (no_line_number, ignore_case) {
            (true, true) => self.no_line_number_caseless(),
            (true, false) => self.no_line_number(),
            (false, true) => self.line_number_caseless(),
            (false, false) => self.line_number(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matcher::MatcherBuilder;
    use std::io::Cursor;

    const LINE: &str = "He started\nmade a run\n& stopped";
    const LINE_MAX_NON_ASCII: &str = "He started again\na\x00nd again\n& AΓain";

    #[test]
    fn max_count_empty() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .no_line_number(false)
            .max_count(Some(0))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let matches = searcher.search_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 0);
    }

    #[test]
    fn max_count_one() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .no_line_number(false)
            .max_count(Some(1))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let matches = searcher.search_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(
            matches.as_ref().unwrap().matches[0],
            &b"He started again"[..]
        );
    }

    #[test]
    fn max_count_large() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "made".to_owned();

        let matcher = MatcherBuilder::new()
            .no_line_number(false)
            .max_count(Some(1000))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let matches = searcher.search_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a run"[..]);
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![1, 2];

        assert!(matches.len() == 2);
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;

        assert!(matches.len() == 2);
        assert_eq!(line_numbers, &LineNumbers::None);
    }
}
