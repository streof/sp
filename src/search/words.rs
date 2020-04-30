use crate::results::{check_words, GenResult};
use crate::search::{GenSearch, Searcher};
use std::io::BufRead;

pub trait Words {
    fn get_matches(&mut self) -> GenResult;
}

impl<'a, R: BufRead> Words for Searcher<'a, R> {
    fn get_matches(&mut self) -> GenResult {
        let (ignore_case, max_count, no_line_number, count) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
            self.matcher.config.count,
        );

        match (no_line_number, ignore_case, max_count, count) {
            (true, true, Some(_), false) => self.no_line_number_caseless_max_count(check_words),
            (true, true, None, false) => self.no_line_number_caseless(check_words),
            (true, false, Some(_), false) => self.no_line_number_max_count(check_words),
            (true, false, None, false) => self.no_line_number(check_words),
            (false, true, Some(_), false) => self.line_number_caseless_max_count(check_words),
            (false, true, None, false) => self.line_number_caseless(check_words),
            (false, false, Some(_), false) => self.line_number_max_count(check_words),
            (false, false, None, false) => self.line_number(check_words),
            (_, true, Some(_), true) => self.cnt_caseless_max_count(check_words),
            (_, true, None, true) => self.cnt_caseless(check_words),
            (_, false, Some(_), true) => self.cnt_max_count(check_words),
            (_, false, None, true) => self.cnt(check_words),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Searcher;
    use crate::matcher::MatcherBuilder;
    use crate::results::{CountResult, GenInnerResult, LineNumbers, SearchResult};
    use std::io::Cursor;

    const LINE: &str = "Gain's\n?gain,\na\x00nd, Gain,\n& AΓain\ngain,";

    #[test]
    fn line_number_starts_with() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(false)
            .starts_with(true)
            .words(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("?gain,".into());
        sr.matches.push("a\x00nd, Gain,".into());
        sr.line_numbers = LineNumbers::Some(vec![2, 3]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn pattern_non_word() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain,".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(false)
            .max_count(Some(2))
            .no_line_number(true)
            .words(true)
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
    fn cnt_starts_with() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .ignore_case(true)
            .no_line_number(false)
            .starts_with(true)
            .words(true)
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
