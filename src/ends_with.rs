use crate::gen_check::GenCheck;
use crate::results::{check_ends_with, GenResult};
use crate::searcher::Searcher;
use std::io::BufRead;

pub trait EndsWithSearch {
    fn get_matches(&mut self) -> GenResult;
}

impl<'a, R: BufRead> EndsWithSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> GenResult {
        let (ignore_case, max_count, no_line_number, count) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
            self.matcher.config.count,
        );

        match (no_line_number, ignore_case, max_count, count) {
            (true, true, Some(_), false) => self.no_line_number_caseless_max_count(check_ends_with),
            (true, true, None, false) => self.no_line_number_caseless(check_ends_with),
            (true, false, Some(_), false) => self.no_line_number_max_count(check_ends_with),
            (true, false, None, false) => self.no_line_number(check_ends_with),
            (false, true, Some(_), false) => self.line_number_caseless_max_count(check_ends_with),
            (false, true, None, false) => self.line_number_caseless(check_ends_with),
            (false, false, Some(_), false) => self.line_number_max_count(check_ends_with),
            (false, false, None, false) => self.line_number(check_ends_with),
            (_, true, Some(_), true) => self.cnt_caseless_max_count(check_ends_with),
            (_, true, None, true) => self.cnt_caseless(check_ends_with),
            (_, false, Some(_), true) => self.cnt_max_count(check_ends_with),
            (_, false, None, true) => self.cnt(check_ends_with),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::MatcherBuilder;
    use crate::results::{CountResult, GenInnerResult, LineNumbers, SearchResult};
    use std::io::Cursor;

    const LINE: &str = "againn\ngain\na\x00nd, again\n& AΓain\nGain";
    const LINE2: &str = "againn\nGain\na\x00nd, aGain\n& AΓain\nGain";

    #[test]
    fn line_number() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
            .ignore_case(false)
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
        sr.matches.push("gain".into());
        sr.matches.push("a\x00nd, again".into());
        sr.line_numbers = LineNumbers::Some(vec![2, 3]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "aγain".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
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
        sr.matches.push("& AΓain".into());
        sr.line_numbers = LineNumbers::Some(vec![4]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE2.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
            .ignore_case(true)
            .no_line_number(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("Gain".into());
        sr.matches.push("a\x00nd, aGain".into());
        sr.matches.push("Gain".into());

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn no_line_number_max_count() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
            .max_count(Some(1))
            .no_line_number(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("gain".into());

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn no_line_number_caseless_max_count() {
        let mut line = Cursor::new(LINE2.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
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
        sr.matches.push("Gain".into());
        sr.matches.push("a\x00nd, aGain".into());

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn cnt_max_count() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .ends_with(true)
            .max_count(Some(3))
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let cr = CountResult { count: 2 };
        assert_eq!(gir, GenInnerResult::Count(cr));
    }
}
