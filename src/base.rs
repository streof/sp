use crate::ext::ByteSliceExt;
use crate::matcher::Matcher;
use crate::matcher::*;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

trait Base {
    fn no_line_number(&mut self) -> MatcherResult;
    fn no_line_number_caseless(&mut self) -> MatcherResult;
    fn line_number(&mut self) -> MatcherResult;
    fn line_number_caseless(&mut self) -> MatcherResult;
}

pub trait BaseMatches {
    fn get_matches(&mut self) -> MatcherResult;
}

// Closures try to borrow `self` as a whole so assign disjoint fields to
// variables first
impl<'a, R: BufRead> Base for Matcher<'a, R> {
    fn no_line_number(&mut self) -> MatcherResult {
        let (reader, pattern) = (&mut self.reader, &self.pattern);

        let Init { mut matches, .. } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if line.contains_str(pattern) {
                matches.push(line.trim_terminator());
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::None)
    }

    fn no_line_number_caseless(&mut self) -> MatcherResult {
        let (reader, pattern) = (&mut self.reader, &self.pattern);

        let Init { mut matches, .. } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            if line.is_ascii() {
                if line.to_ascii_lowercase().contains_str(pattern) {
                    matches.push(line.trim_terminator());
                }
            } else {
                let mut buf = vec![];
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    matches.push(line.trim_terminator());
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::None)
    }

    fn line_number(&mut self) -> MatcherResult {
        let (reader, pattern) = (&mut self.reader, &self.pattern);

        let Init {
            mut matches,
            mut line_numbers_inner,
            mut line_number,
        } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            if line.contains_str(pattern) {
                matches.push(line.trim_terminator());
                line_numbers_inner.push(line_number);
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }

    fn line_number_caseless(&mut self) -> MatcherResult {
        let (reader, pattern) = (&mut self.reader, &self.pattern);

        let Init {
            mut matches,
            mut line_numbers_inner,
            mut line_number,
        } = Init::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            if line.is_ascii() {
                if line.to_ascii_lowercase().contains_str(pattern) {
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            } else {
                let mut buf = vec![];
                line.to_lowercase_into(&mut buf);
                if buf.contains_str(pattern) {
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }
}

impl<'a, R: BufRead> BaseMatches for Matcher<'a, R> {
    fn get_matches(&mut self) -> MatcherResult {
        let (ignore_case, no_line_number) = (self.config.ignore_case, self.config.no_line_number);

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

    use crate::cli::ConfigBuilder;
    use std::io::Cursor;

    const LINE: &str = "He started\nmade a run\n& stopped";
    const LINE_BIN: &str = "He started\nmad\x00e a run\n& stopped";
    const LINE_BIN2: &str = "He started\r\nmade a r\x00un\n& stopped";
    const LINE_BIN3: &str = "He started\r\nmade a r\x00un\r\n& stopped";
    const LINE_MAX_NON_ASCII: &str = "He started again\na\x00nd again\n& AΓain";

    #[test]
    fn find_no_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"Made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(true)
            .max_count(None)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = MatcherType::Base(matcher).find_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.is_empty());
        assert_eq!(line_numbers, &LineNumbers::None);
    }

    #[test]
    fn find_a_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(None)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = MatcherType::Base(matcher).find_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 1);
        assert_eq!(matches[0], &b"made a run"[..]);
        let line_number_inner: Vec<u64> = vec![2];
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn search_binary_text() {
        let mut line = Cursor::new(LINE_BIN.as_bytes());
        let pattern = &"made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(None)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = MatcherType::Base(matcher).find_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 0);
    }

    #[test]
    fn search_binary_text2() {
        let mut line = Cursor::new(LINE_BIN2.as_bytes());
        let pattern = &"made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(None)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = MatcherType::Base(matcher).find_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a r\x00un"[..]);
    }

    #[test]
    fn search_binary_text3() {
        let mut line = Cursor::new(LINE_BIN3.as_bytes());
        let pattern = &"r\x00un".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(None)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = MatcherType::Base(matcher).find_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a r\x00un"[..]);
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(false)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = MatcherType::Base(matcher).find_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 2);
        let line_number_inner: Vec<u64> = vec![1, 2];
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = &"aγain".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(true)
            .build();

        let matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = MatcherType::Base(matcher).find_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 1);
        assert_eq!(line_numbers, &LineNumbers::None);
    }
}
