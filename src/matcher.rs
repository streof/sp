use bstr::{io::BufReadExt, BString, ByteSlice};
use std::io::BufRead;

pub struct Matcher<'a, R> {
    pub reader: R,
    pub pattern: &'a str,
    pub no_line_number: &'a bool,
}

pub struct MatchResult {
    pub matches: Vec<BString>,
}

pub type MatcherResult = Result<MatchResult, std::io::Error>;

impl<'a, R: BufRead> Matcher<'a, R> {
    pub fn get_matches(&mut self) -> MatcherResult {
        // Closures try to borrow `self` as a whole
        // So assign disjoint fields to variables first
        let (reader, pattern) = (&mut self.reader, &self.pattern);
        let mut matches = Vec::new();
        reader.for_byte_line_with_terminator(|line| {
            if line.contains_str(pattern) {
                matches.push(line.into()); // convert to BString first
            }
            Ok(true)
        })?;
        let match_result = MatchResult { matches };
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
        let no_line_number = &false;

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            no_line_number,
        };

        let matches = matcher.get_matches();

        assert!(matches.as_ref().unwrap().matches.is_empty());
    }

    #[test]
    fn find_a_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = &"made".to_owned();
        let no_line_number = &false;

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            no_line_number,
        };

        let matches = matcher.get_matches();

        assert!(matches.as_ref().unwrap().matches.len() == 1);
        assert_eq!(matches.as_ref().unwrap().matches[0], &b"made a run\n"[..]);
    }

    #[test]
    fn search_binary_text() {
        let mut line = Cursor::new(LINE_BIN.as_bytes());
        let pattern = &"made".to_owned();
        let no_line_number = &false;

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            no_line_number,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 0);
    }

    #[test]
    fn search_binary_text2() {
        let mut line = Cursor::new(LINE_BIN2.as_bytes());
        let pattern = &"made".to_owned();
        let no_line_number = &false;

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            no_line_number,
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
        let no_line_number = &false;

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
            no_line_number,
        };

        let matches = matcher.get_matches();

        assert_eq!(matches.as_ref().unwrap().matches.len(), 1);
        assert_eq!(
            matches.as_ref().unwrap().matches[0],
            &b"made a r\x00un\r\n"[..]
        );
    }
}
