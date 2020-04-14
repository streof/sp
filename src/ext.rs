use bstr::{BString, ByteSlice};
use std::str;

pub trait ByteSliceExt {
    fn trim_terminator(&self) -> BString;
    fn check_starts_with(&self, pattern: &str) -> bool;
}

impl ByteSliceExt for [u8] {
    /// Trims line terminator and converts result to BString
    fn trim_terminator(&self) -> BString {
        self.trim_end_with(|c| c == '\n' || c == '\r').into()
    }

    /// Checks if line contains words starting with provided pattern.
    fn check_starts_with(&self, pattern: &str) -> bool {
        self.trim_terminator()
            .split_str(" ")
            .any(|word| word.starts_with_str(pattern))
    }
}

pub trait BStringExt {
    fn to_utf8(&self) -> &str;
}

impl BStringExt for BString {
    fn to_utf8(&self) -> &str {
        str::from_utf8(self).expect("Found invalid UTF-8")
    }
}
