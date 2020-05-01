use crate::results::{check_contains, CountResult, GenResult, SearchInnerResult, Upcast};
use crate::search::Searcher;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::BufRead;

trait MaxCountSearch {
    fn no_line_number(&mut self) -> GenResult;
    fn no_line_number_caseless(&mut self) -> GenResult;
    fn line_number(&mut self) -> GenResult;
    fn line_number_caseless(&mut self) -> GenResult;
    fn cnt(&mut self) -> GenResult;
    fn cnt_caseless(&mut self) -> GenResult;
}

pub trait MaxCount {
    fn get_matches(&mut self) -> GenResult;
}

impl<'a, R: BufRead> MaxCountSearch for Searcher<'a, R> {
    fn cnt(&mut self) -> GenResult {
        let (reader, pattern, matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
            self.matcher.config.max_count.unwrap(),
        );

        let mut cr = CountResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            cr.check_and_add(pattern, line, check_contains);
            Ok(true)
        })?;

        cr.upcast()
    }

    fn cnt_caseless(&mut self) -> GenResult {
        let (reader, pattern, matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
            self.matcher.config.max_count.unwrap(),
        );

        let mut cr = CountResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
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
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
            self.matcher.config.max_count.unwrap(),
        );

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            sir.check_and_store_nln_max_count(pattern, line, &mut matches_left, check_contains);
            Ok(true)
        })?;

        sir.upcast()
    }

    fn no_line_number_caseless(&mut self) -> GenResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
            self.matcher.config.max_count.unwrap(),
        );

        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            if line.is_ascii() {
                sir.check_and_store_separate_nln_max_count(
                    pattern,
                    &line.to_ascii_lowercase(),
                    line,
                    &mut matches_left,
                    check_contains,
                );
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                sir.check_and_store_separate_nln_max_count(
                    pattern,
                    &buf,
                    line,
                    &mut matches_left,
                    check_contains,
                );
            }
            Ok(true)
        })?;

        sir.upcast()
    }

    fn line_number(&mut self) -> GenResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
            self.matcher.config.max_count.unwrap(),
        );

        let mut line_number = 0;
        let mut sir = SearchInnerResult::default();

        reader.for_byte_line_with_terminator(|line| {
            if matches_left == 0 as u64 {
                return Ok(true);
            }
            line_number += 1;
            sir.check_and_store_max_count(
                pattern,
                line_number,
                line,
                &mut matches_left,
                check_contains,
            );
            Ok(true)
        })?;

        sir.upcast()
    }

    fn line_number_caseless(&mut self) -> GenResult {
        let (reader, pattern, mut matches_left) = (
            &mut self.reader,
            self.matcher.pattern.as_bytes(),
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
                sir.check_and_store_separate_max_count(
                    pattern,
                    line_number,
                    &line.to_ascii_lowercase(),
                    line,
                    &mut matches_left,
                    check_contains,
                );
            } else {
                let mut buf = Default::default();
                line.to_lowercase_into(&mut buf);
                sir.check_and_store_separate_max_count(
                    pattern,
                    line_number,
                    &buf,
                    line,
                    &mut matches_left,
                    check_contains,
                );
            }
            Ok(true)
        })?;

        sir.upcast()
    }
}

impl<'a, R: BufRead> MaxCount for Searcher<'a, R> {
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
    const LINE_MAX_NON_ASCII: &str = "He started again\na\x00nd again\n& AΓain";

    #[test]
    fn max_count_empty() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(Some(0))
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
    fn max_count_one() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = "again".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(Some(1))
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
        sr.line_numbers = LineNumbers::Some(vec![1]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn max_count_large() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "made".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(Some(1000))
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

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("He started again".into());
        sr.matches.push("a\x00nd again".into());

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn cnt_max_count_zero() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = " ".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .max_count(Some(0))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let cr = CountResult { count: 0 };
        assert_eq!(gir, GenInnerResult::Count(cr));
    }

    #[test]
    fn cnt_max_count_two() {
        let mut line = Cursor::new(LINE_MAX_NON_ASCII.as_bytes());
        let pattern = " ".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .max_count(Some(2))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let cr = CountResult { count: 3 };
        assert_eq!(gir, GenInnerResult::Count(cr));
    }
}
