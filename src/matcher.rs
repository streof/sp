use crate::cli::Config;
use bstr::{io::BufReadExt, BString, ByteSlice};
use std::io::BufRead;

pub struct Matcher<'a, R> {
    pub reader: R,
    pub pattern: &'a str,
    pub config: &'a Config,
}

pub struct MatchResult {
    pub matches: Vec<BString>,
    pub line_numbers: Option<Vec<u64>>,
}

pub type MatcherResult = Result<MatchResult, std::io::Error>;

impl<'a, R: BufRead> Matcher<'a, R> {
    pub fn get_matches(&mut self) -> MatcherResult {
        // Closures try to borrow `self` as a whole
        // So assign disjoint fields to variables first
        let (reader, pattern, no_line_number) =
            (&mut self.reader, &self.pattern, self.config.no_line_number);
        let mut matches = vec![];
        let mut line_numbers_inner = vec![];

        // Find and store matches (and line numbers if required) in a vec
        let line_numbers = if no_line_number {
            reader.for_byte_line_with_terminator(|line| {
                if line.contains_str(pattern) {
                    matches.push(line.into());
                }
                Ok(true)
            })?;
            None
        } else {
            let mut line_number: u64 = 0;
            reader.for_byte_line_with_terminator(|line| {
                line_number += 1;
                if line.contains_str(pattern) {
                    matches.push(line.into()); // convert to BString first
                    line_numbers_inner.push(line_number);
                }
                Ok(true)
            })?;
            Some(line_numbers_inner)
        };

        let match_result = MatchResult {
            matches,
            line_numbers,
        };
        Ok(match_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    const LINE: &str = "He started\nmade a run\n& stopped";
    const LINE_BIN: &str = "He started\nmad\x00e a run\n& stopped";
    const LINE_BIN2: &str = "He started\r\nmade a r\x00un\n& stopped";
    const LINE_BIN3: &str = "He started\r\nmade a r\x00un\r\n& stopped";

    #[test]
    fn find_no_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"Made".to_owned();

        let config = Config {
            no_line_number: true,
        };

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
        assert_eq!(line_numbers.is_none(), true);
    }

    #[test]
    fn find_a_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"made".to_owned();

        let config = Config {
            no_line_number: false,
        };

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matcher_result = matcher.get_matches();
        let match_result = matcher_result.as_ref().unwrap();
        let matches = &match_result.matches;
        let line_numbers = match_result.line_numbers.as_ref();

        assert!(matches.len() == 1);
        assert_eq!(matches[0], &b"made a run\n"[..]);
        let line_number_inner: Vec<u64> = vec![2];
        assert_eq!(line_numbers, Some(&line_number_inner));
    }

    #[test]
    fn search_binary_text() {
        let mut line = Cursor::new(LINE_BIN.as_bytes());
        let pattern = &"made".to_owned();

        let config = Config {
            no_line_number: false,
        };

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

        let config = Config {
            no_line_number: false,
        };

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(
            matches.as_ref().unwrap().matches[0],
            &b"made a r\x00un\n"[..]
        );
    }

    #[test]
    fn search_binary_text3() {
        let mut line = Cursor::new(LINE_BIN3.as_bytes());
        let pattern = &"r\x00un".to_owned();

        let config = Config {
            no_line_number: false,
        };

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            config: &config,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(
            matches.as_ref().unwrap().matches[0],
            &b"made a r\x00un\r\n"[..]
        );
    }
}
