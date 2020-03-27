use crate::cli::CliResult;
use crate::matcher::{MatchResult, MatcherResult};
use std::io::Write;
use std::str;

pub struct Writer<W> {
    pub wrt: W,
}

impl<W: Write> Writer<W> {
    pub fn print_matches(mut self, matcher_result: MatcherResult) -> CliResult {
        match matcher_result {
            Ok(match_result) => self.print_lines_iter(match_result).expect("Error occured"),
            Err(_) => println!("Error occured"),
        };
        Ok(())
    }

    fn print_lines_iter(&mut self, match_result: MatchResult) -> CliResult {
        let matches = match_result.matches;
        for line in matches.iter() {
            writeln!(
                self.wrt,
                "{}",
                str::from_utf8(line.as_slice())
                    .expect("Found invalid UTF-8")
                    .trim_end()
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::matcher::*;
    use std::fs::File;
    use std::io::Cursor;
    use std::io::{Read, Seek, SeekFrom, Write};

    const DICKENS: &str = "\
He started      \r
make a run
& stopped.
He started
made a quick run
and stopped
He started
made a RuN
and then stopped\
";

    #[test]
    fn text_output() {
        let expected = "\
make a run
made a quick run
";
        // Build matcher
        let mut matcher = Matcher {
            reader: &mut Cursor::new(DICKENS.as_bytes()),
            pattern: &"run".to_owned(),
            no_line_number: &false,
        };
        let matches = matcher.get_matches();

        // Write to temp file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        let wrt = Writer {
            wrt: Write::by_ref(&mut tmpfile),
        };
        wrt.print_matches(matches).unwrap();

        // Seek to start (!)
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read back
        let mut got = String::new();
        tmpfile.read_to_string(&mut got).unwrap();

        assert_eq!(expected, got);
    }
}
