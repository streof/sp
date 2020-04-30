#![warn(
    clippy::nursery,
    missing_debug_implementations,
    missing_docs,
    // unreachable_pub
)]
#![forbid(unsafe_code)]
// #![warn(clippy::pedantic)]

//! This library implements all functionalities provided by `grrs`. It relies
//! on the following crates:
//!
//! * `bstr`: string oriented methods for byte strings: similar to
//! Unicode strings but *not guaranteed* to be valid UTF-8.
//! * `anyhow`: convenient and idiomatic error handling
//! * `structopt`: parsing command line arguments and many additional features
pub mod cli;
pub(crate) mod ext;
pub(crate) mod matcher;
pub(crate) mod results;
pub(crate) mod search;
pub(crate) mod writer;
