use bstr::{io::BufReadExt, BString, ByteSlice};
use std::io::BufRead;

pub struct Matcher<'a, R> {
    pub reader: R,
    pub pattern: &'a str,
}

impl<'a, R: BufRead> Matcher<'a, R> {
    pub fn get_matches(&mut self) -> Result<Vec<BString>, std::io::Error> {
        // Closures try to borrow `self` as a whole
        // So assign disjoint fields to variables first
        let reader = &mut self.reader;
        let pattern = &self.pattern;
        let mut matches = Vec::new();
        reader.for_byte_line_with_terminator(|line| {
            if line.contains_str(pattern) {
                matches.push(line.into()); // convert to BString first
            }
            Ok(true)
        })?;
        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::*;
    use std::io::Cursor;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn find_a_match_file() {
        let file = "tests/data/sample.txt";
        let mut cli = Cli {
            path: PathBuf::from_str(file).unwrap(),
            pattern: "lorem".to_owned(),
            no_line_number: false,
        };

        let reader = &std::fs::read(file).unwrap()[..];
        let mut result = Vec::new();
        cli.show_matches(reader, &mut result).unwrap();
        assert_eq!(result, b"lorem ipsum\n")
    }

    // TODO write a few more tests
    #[test]
    fn find_a_match_line() {
        let mut line = Cursor::new(b"You, good Cornelius, and you");
        let pattern = &"you".to_owned();

        let mut matcher = Matcher {
            reader: &mut line,
            pattern,
        };

        assert_eq!(
            matcher.get_matches().unwrap()[0],
            &b"You, good Cornelius, and you"[..]
        )
    }
}
