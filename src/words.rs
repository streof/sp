use crate::gen_check::GenCheck;
use crate::results::{check_words, GenResult};
use crate::searcher::Searcher;
use std::io::BufRead;

pub trait WordsSearch {
    fn get_matches(&mut self) -> GenResult;
}

impl<'a, R: BufRead> WordsSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> GenResult {
        let (ignore_case, max_count, no_line_number) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
        );

        match (no_line_number, ignore_case, max_count) {
            (true, true, Some(_)) => self.no_line_number_caseless_max_count(check_words),
            (true, true, None) => self.no_line_number_caseless(check_words),
            (true, false, Some(_)) => self.no_line_number_max_count(check_words),
            (true, false, None) => self.no_line_number(check_words),
            (false, true, Some(_)) => self.line_number_caseless_max_count(check_words),
            (false, true, None) => self.line_number_caseless(check_words),
            (false, false, Some(_)) => self.line_number_max_count(check_words),
            (false, false, None) => self.line_number(check_words),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matcher::MatcherBuilder;
    use crate::results::{GenInnerResult, LineNumbers};
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

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();

        if let GenInnerResult::Search(search_result) = search_result {
            let matches = &search_result.matches;
            let line_numbers = &search_result.line_numbers;
            let line_number_inner: Vec<u64> = vec![2, 3];

            assert!(matches.len() == 2);
            assert_eq!(matches[0], &b"?gain,"[..]);
            assert_eq!(matches[1], &b"a\x00nd, Gain,"[..]);
            assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
        };
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

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();

        if let GenInnerResult::Search(search_result) = search_result {
            let matches = &search_result.matches;

            assert!(matches.is_empty());
        };
    }
}
