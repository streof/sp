use crate::results::{check_contains, CountResult, GenResult, SearchInnerResult, Upcast};
use crate::search::Searcher;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

trait BaseSearch {
    fn no_line_number(&mut self) -> GenResult;
    fn no_line_number_caseless(&mut self) -> GenResult;
    fn line_number(&mut self) -> GenResult;
    fn line_number_caseless(&mut self) -> GenResult;
    fn cnt(&mut self) -> GenResult;
    fn cnt_caseless(&mut self) -> GenResult;
}

pub trait Base {
    fn get_matches(&mut self) -> GenResult;
}

// Closures try to borrow `self` as a whole so assign disjoint fields to
// variables first
impl<'a, R: BufRead> BaseSearch for Searcher<'a, R> {
    fn cnt(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut cr = CountResult::default();

        // TODO: The underlying buffer can be improved
        reader.for_byte_line_with_terminator(|line| {
            // TODO: Replace pattern by pattern.as_bytes() and corresponding
            // function implementation
            cr.check_and_add(pattern, line, check_contains);
            Ok(true)
        })?;

        cr.upcast()
    }

    fn cnt_caseless(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut cr = CountResult::default();

        // TODO: Redundant second pass over same line
        reader.for_byte_line_with_terminator(|line| {
            if line.is_ascii() {
                cr.check_and_add(pattern, &line.to_ascii_lowercase(), check_contains);
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                cr.check_and_add(pattern, &buf, check_contains);
            }
            Ok(true)
        })?;

        cr.upcast()
    }

    fn no_line_number(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            sir.check_and_store_nln(pattern, line, check_contains);
            Ok(true)
        })?;

        sir.upcast()
    }

    fn no_line_number_caseless(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if line.is_ascii() {
                sir.check_and_store_separate_nln(
                    pattern,
                    &line.to_ascii_lowercase(),
                    line,
                    check_contains,
                );
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                sir.check_and_store_separate_nln(pattern, &buf, line, check_contains);
            }
            Ok(true)
        })?;

        sir.upcast()
    }

    fn line_number(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            sir.check_and_store(pattern, line_number, line, check_contains);
            Ok(true)
        })?;

        sir.upcast()
    }

    fn line_number_caseless(&mut self) -> GenResult {
        let (reader, pattern) = (&mut self.reader, self.matcher.pattern.as_bytes());

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            line_number += 1;
            if line.is_ascii() {
                sir.check_and_store_separate(
                    pattern,
                    line_number,
                    &line.to_ascii_lowercase(),
                    line,
                    check_contains,
                );
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                sir.check_and_store_separate(pattern, line_number, &buf, line, check_contains);
            }
            Ok(true)
        })?;

        sir.upcast()
    }
}

impl<'a, R: BufRead> Base for Searcher<'a, R> {
    fn get_matches(&mut self) -> GenResult {
        let (ignore_case, no_line_number, count) = (
            self.matcher.config.ignore_case,
            self.matcher.config.no_line_number,
            self.matcher.config.count,
        );

        match (no_line_number, ignore_case, count) {
            (true, true, false) => self.no_line_number_caseless(),
            (true, false, false) => self.no_line_number(),
            (false, true, false) => self.line_number_caseless(),
            (false, false, false) => self.line_number(),
            (_, true, true) => self.cnt_caseless(),
            (_, false, true) => self.cnt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CountResult, Searcher};
    use crate::matcher::MatcherBuilder;
    use crate::results::{GenInnerResult, LineNumbers, SearchResult};
    use std::io::Cursor;

    const LINE: &str = "He started\nmade a run\n& stopped";
    const LINE_BIN: &str = "He started\nmad\x00e a run\n& stopped";
    const LINE_BIN2: &str = "He started\r\nmade a r\x00un\n& stopped";
    const LINE_BIN3: &str = "He started\r\nmade a r\x00un\r\n& stopped";
    const LINE_MAX_NON_ASCII: &str = "He started again\na\x00nd again\n& AΓain";

    #[test]
    fn find_no_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "Made".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(None)
            .no_line_number(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let sr = SearchResult::default();

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn find_a_match() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "made".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(None)
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("made a run".into());
        sr.line_numbers = LineNumbers::Some(vec![2]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn search_binary_text() {
        let mut line = Cursor::new(LINE_BIN.as_bytes());
        let pattern = "made".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(None)
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let sr = SearchResult::default();

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn search_binary_text2() {
        let mut line = Cursor::new(LINE_BIN2.as_bytes());
        let pattern = "made".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(None)
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("made a r\x00un".into());
        sr.line_numbers = LineNumbers::Some(vec![2]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn search_binary_text3() {
        let mut line = Cursor::new(LINE_BIN3.as_bytes());
        let pattern = "r\x00un".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(None)
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("made a r\x00un".into());
        sr.line_numbers = LineNumbers::Some(vec![2]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(false)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("He started again".into());
        sr.matches.push("a\x00nd again".into());
        sr.line_numbers = LineNumbers::Some(vec![1, 2]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "aγain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("& AΓain".into());

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn cnt() {
        let mut line = Cursor::new(LINE_BIN3.as_bytes());
        let pattern = "t".to_owned();

        let matcher = MatcherBuilder::new().count(true).build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let cr = CountResult { count: 2 };
        assert_eq!(gir, GenInnerResult::Count(cr));
    }

    #[test]
    fn cnt_caseless() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "γ".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .ignore_case(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let cr = CountResult { count: 1 };
        assert_eq!(gir, GenInnerResult::Count(cr));
    }
}
