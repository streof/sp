use bstr::{BString, ByteSlice};
use std::str;

pub trait ByteSliceExt {
    fn trim_terminator(&self) -> BString;
}

impl ByteSliceExt for [u8] {
    /// Trims line terminator and converts result to `BString`
    fn trim_terminator(&self) -> BString {
        self.trim_end_with(|c| c == '\n' || c == '\r').into()
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
