use crate::gen_check::GenCheck;
use crate::search_inner::*;
use crate::searcher::*;
use std::io::BufRead;

pub trait StartsWithSearch {
    fn get_matches(&mut self) -> SearcherResult;
}

impl<'a, R: BufRead> StartsWithSearch for Searcher<'a, R> {
    fn get_matches(&mut self) -> SearcherResult {
        let (ignore_case, max_count, no_line_number) = (
            self.matcher.config.ignore_case,
            self.matcher.config.max_count,
            self.matcher.config.no_line_number,
        );

        match (no_line_number, ignore_case, max_count) {
            (true, true, Some(_)) => self.no_line_number_caseless_max_count(check_starts_with),
            (true, true, None) => self.no_line_number_caseless(check_starts_with),
            (true, false, Some(_)) => self.no_line_number_max_count(check_starts_with),
            (true, false, None) => self.no_line_number(check_starts_with),
            (false, true, Some(_)) => self.line_number_caseless_max_count(check_starts_with),
            (false, true, None) => self.line_number_caseless(check_starts_with),
            (false, false, Some(_)) => self.line_number_max_count(check_starts_with),
            (false, false, None) => self.line_number(check_starts_with),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matcher::MatcherBuilder;
    use std::io::Cursor;

    const LINE: &str = "again\na\tgain\na\x00nd, gain\n&\u{2003}AΓain\nGain";
    const LINE2: &str = "again\nGain\na\x00nd, gain\n& AΓain\nGain";

    #[test]
    fn line_number() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(false)
            .max_count(Some(2))
            .no_line_number(false)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![2, 3];

        assert!(matches.len() == 2);
        assert_eq!(matches[0], &b"a\tgain"[..]);
        assert_eq!(matches[1], &b"a\x00nd, gain"[..]);
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn line_number_caseless() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "aγain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(None)
            .no_line_number(false)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;
        let line_number_inner: Vec<u64> = vec![4];

        assert!(matches.len() == 1);
        assert_eq!(matches[0], "&\u{2003}AΓain".as_bytes());
        assert_eq!(line_numbers, &LineNumbers::Some(line_number_inner));
    }

    #[test]
    fn no_line_number_caseless() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .no_line_number(true)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;

        assert!(matches.len() == 3);
        assert_eq!(matches[0], &b"a\tgain"[..]);
        assert_eq!(matches[1], &b"a\x00nd, gain"[..]);
        assert_eq!(matches[2], &b"Gain"[..]);
        assert_eq!(line_numbers, &LineNumbers::None);
    }

    #[test]
    fn no_line_number_max_count() {
        let mut line = Cursor::new(LINE.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .max_count(Some(2))
            .no_line_number(true)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;

        assert!(matches.len() == 2);
        assert_eq!(matches[0], &b"a\tgain"[..]);
        assert_eq!(matches[1], &b"a\x00nd, gain"[..]);
        assert_eq!(line_numbers, &LineNumbers::None);
    }

    #[test]
    fn no_line_number_caseless_max_count() {
        let mut line = Cursor::new(LINE2.as_bytes());
        let pattern = "gain".to_owned();

        let matcher = MatcherBuilder::new()
            .ignore_case(true)
            .max_count(Some(2))
            .no_line_number(true)
            .starts_with(true)
            .build(pattern);

        let searcher = Searcher {
            reader: &mut line,
            matcher: &matcher,
        };

        let searcher_result = searcher.search_matches();
        let search_result = searcher_result.as_ref().unwrap();
        let matches = &search_result.matches;
        let line_numbers = &search_result.line_numbers;

        assert!(matches.len() == 2);
        assert_eq!(matches[0], &b"Gain"[..]);
        assert_eq!(matches[1], &b"a\x00nd, gain"[..]);
        assert_eq!(line_numbers, &LineNumbers::None);
    }
}
