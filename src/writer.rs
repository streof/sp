use crate::cli::Output;
use crate::ext::BStringExt;
use crate::matcher::Config;
use crate::results::{CountResult, GenInnerResult, GenResult, LineNumbers, SearchResult};
use std::io::Write;

#[derive(Debug)]
pub struct Writer<W> {
    pub wrt: W,
}

impl<W: Write> Writer<W> {
    pub fn print_matches(mut self, gen_result: GenResult, config: &Config) -> Output {
        if let Ok(match_result) = gen_result {
            self.print_lines_iter(match_result, config)?
        } else {
            eprintln!("This error should never occur")
        };
        Ok(())
    }

    fn print_lines_iter(&mut self, gir: GenInnerResult, config: &Config) -> Output {
        let nln = config.no_line_number;
        match gir {
            GenInnerResult::Count(count) => self.print_count(&count),
            GenInnerResult::Search(search) => self.print_search(search, nln),
        }
    }

    fn print_count(&mut self, count: &CountResult) -> Output {
        if count.count == 0 {
            Ok(())
        } else {
            Ok(writeln!(self.wrt, "{}", count.count)?)
        }
    }

    fn print_search(&mut self, search: SearchResult, nln: bool) -> Output {
        let matches = search.matches;
        let line_numbers = search.line_numbers;
        if nln {
            for single_match in &matches {
                writeln!(self.wrt, "{}", BStringExt::to_utf8(single_match))?;
            }
        } else if let LineNumbers::Some(lni) = line_numbers {
            for (line_number, single_match) in lni.iter().zip(matches) {
                writeln!(
                    self.wrt,
                    "{}:{}",
                    line_number,
                    BStringExt::to_utf8(&single_match)
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::Writer;
    use crate::matcher::MatcherBuilder;
    use crate::search::Searcher;
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
    fn print_dickens() {
        let expected = "\
2:make a run
5:made a quick run
";
        // Build config and matcher
        let pattern = "run".to_owned();
        let matcher = MatcherBuilder::new()
            .no_line_number(false)
            .max_count(None)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut Cursor::new(DICKENS.as_bytes()),
            matcher: &matcher,
        };

        let matches = searcher.search_matches();

        // Write to temp file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        let wrt = Writer {
            wrt: Write::by_ref(&mut tmpfile),
        };
        wrt.print_matches(matches, &matcher.config).unwrap();

        // Seek to start (!)
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read back
        let mut got = String::new();
        tmpfile.read_to_string(&mut got).unwrap();

        assert_eq!(expected, got);
    }

    #[test]
    fn print_dickens_no_line_number() {
        let expected = "\
make a run
";
        // Build config and matcher
        let pattern = "run".to_owned();
        let matcher = MatcherBuilder::new()
            .no_line_number(true)
            .max_count(Some(1))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut Cursor::new(DICKENS.as_bytes()),
            matcher: &matcher,
        };
        let matches = searcher.search_matches();

        // Write to temp file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        let wrt = Writer {
            wrt: Write::by_ref(&mut tmpfile),
        };
        wrt.print_matches(matches, &matcher.config).unwrap();

        // Seek to start (!)
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read back
        let mut got = String::new();
        tmpfile.read_to_string(&mut got).unwrap();

        assert_eq!(expected, got);
    }
}
