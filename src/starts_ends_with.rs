use crate::gen_check::GenCheck;
use crate::results::{check_starts_ends_with, GenResult};
use crate::searcher::Searcher;
use std::io::BufRead;

pub trait StartsEndsWithSearch {
    fn get_matches(&mut self) -> GenResult;
}

impl<'a, R: BufRead> StartsEndsWithSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> GenResult {
        let (ignore_case, max_count, no_line_number, count) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
            self.matcher.config.count,
        );

        match (no_line_number, ignore_case, max_count, count) {
            (true, true, Some(_), false) => {
                self.no_line_number_caseless_max_count(check_starts_ends_with)
            }
            (true, true, None, false) => self.no_line_number_caseless(check_starts_ends_with),
            (true, false, Some(_), false) => self.no_line_number_max_count(check_starts_ends_with),
            (true, false, None, false) => self.no_line_number(check_starts_ends_with),
            (false, true, Some(_), false) => {
                self.line_number_caseless_max_count(check_starts_ends_with)
            }
            (false, true, None, false) => self.line_number_caseless(check_starts_ends_with),
            (false, false, Some(_), false) => self.line_number_max_count(check_starts_ends_with),
            (false, false, None, false) => self.line_number(check_starts_ends_with),
            (_, true, Some(_), true) => self.cnt_caseless_max_count(check_starts_ends_with),
            (_, true, None, true) => self.cnt_caseless(check_starts_ends_with),
            (_, false, Some(_), true) => self.cnt_max_count(check_starts_ends_with),
            (_, false, None, true) => self.cnt(check_starts_ends_with),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::MatcherBuilder;
    use crate::results::{CountResult, GenInnerResult, LineNumbers, SearchResult};
    use std::io::Cursor;

    const LINE: &str = "gain\ngain,\na\x00nd, Gain,\n& AΓain\ngain,";

    #[test]
    fn line_number() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain,".to_owned();

        let matcher = MatcherBuilder::new()
            .ends_with(true)
            .ignore_case(false)
            .max_count(Some(1))
            .no_line_number(false)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let gen_result = searcher.search_matches();
        let gir = gen_result.unwrap();

        let mut sr = SearchResult::default();
        sr.matches.push("gain,".into());
        sr.line_numbers = LineNumbers::Some(vec![2]);

        assert_eq!(gir, GenInnerResult::Search(sr));
    }

    #[test]
    fn cnt_caseless_max_count() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain,".to_owned();

        let matcher = MatcherBuilder::new()
            .count(true)
            .ends_with(true)
            .ignore_case(true)
            .max_count(Some(1))
            .starts_with(true)
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
