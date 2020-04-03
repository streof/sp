use crate::cli::Config;
use crate::ext::ByteSliceExt;
use bstr::{io::BufReadExt, BString, ByteSlice};
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

// Closures try to borrow `self` as a whole so assign disjoint fields to
// variables first
impl<'a, R: BufRead> Matcher<'a, R> {
    /// Convenient method for wrapping `Vec<BString>` and `LineNumbers` before
    /// returning as `MatcherResult`
    pub fn ret_matcher_result(matches: Vec<BString>, line_numbers: LineNumbers) -> MatcherResult {
        let match_result = MatchResult {
            matches,
            line_numbers,
        };

        Ok(match_result)
    }

    pub fn limit_no_line_number(&mut self) -> MatcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.pattern,
            self.config.max_count.unwrap(),
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

        Self::ret_matcher_result(matches, LineNumbers::None)
    }

    pub fn limit_no_line_number_caseless(&mut self) -> MatcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.pattern,
            self.config.max_count.unwrap(),
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
                if line.to_lowercase().contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::None)
    }

    pub fn limit_line_number(&mut self) -> MatcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.pattern,
            self.config.max_count.unwrap(),
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

        Self::ret_matcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }

    pub fn limit_line_number_caseless(&mut self) -> MatcherResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            &self.pattern,
            self.config.max_count.unwrap(),
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
                if line.to_lowercase().contains_str(pattern) {
                    matches_left -= 1;
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }

    pub fn no_line_number(&mut self) -> MatcherResult {
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

    pub fn no_line_number_caseless(&mut self) -> MatcherResult {
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
                if line.to_lowercase().contains_str(pattern) {
                    matches.push(line.trim_terminator());
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::None)
    }

    pub fn line_number(&mut self) -> MatcherResult {
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

    pub fn line_number_caseless(&mut self) -> MatcherResult {
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
                if line.to_lowercase().contains_str(pattern) {
                    matches.push(line.trim_terminator());
                    line_numbers_inner.push(line_number);
                }
            }
            Ok(true)
        })?;

        Self::ret_matcher_result(matches, LineNumbers::Some(line_numbers_inner))
    }

    pub fn get_matches(&mut self) -> MatcherResult {
        let (ignore_case, max_count, no_line_number) = (
            self.config.ignore_case,
            self.config.max_count,
            self.config.no_line_number,
        );

        match (max_count.is_some(), no_line_number, ignore_case) {
            (true, true, true) => self.limit_no_line_number_caseless(),
            (true, true, false) => self.limit_no_line_number(),
            (true, false, true) => self.limit_line_number_caseless(),
            (true, false, false) => self.limit_line_number(),
            (false, true, true) => self.no_line_number_caseless(),
            (false, true, false) => self.no_line_number(),
            (false, false, true) => self.line_number_caseless(),
            (false, false, false) => self.line_number(),
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
    const LINE_MAX: &str = "He started again\nand again\n& Again";

    #[test]
    fn find_no_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"Made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(true)
            .max_count(None)
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
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

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
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

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

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

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

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

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a r\x00un"[..]);
    }

    #[test]
    fn max_count_empty() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(Some(0))
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 0);
    }

    #[test]
    fn max_count_one() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(Some(1))
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(
            matches.as_ref().unwrap().matches[0],
            &b"He started again"[..]
        );
    }

    #[test]
    fn max_count_large() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"made".to_owned();

        let config = ConfigBuilder::new()
            .no_line_number(false)
            .max_count(Some(1000))
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a run"[..]);
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(false)
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 3);
        let line_number_inner: Vec<u64> = vec![1, 2, 3];
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(true)
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 3);
        assert_eq!(line_numbers, &LineNumbers::None);
    }

    #[test]
    fn limit_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(false)
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![1, 2];

        assert!(matches.len() == 2);
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn limit_no_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX.as_bytes());
        let pattern = &"again".to_owned();

        let config = ConfigBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(true)
            .build();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = &match_result.line_numbers;

        assert!(matches.len() == 2);
        assert_eq!(line_numbers, &LineNumbers::None);
    }
}
