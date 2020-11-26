//! A library to clean up music metadata such as artist, album and song names.
//! This is a Rust port of the [metadata-filter](https://github.com/web-scrobbler/metadata-filter)
//! TypeScript library.
//!
//! Currently, this library is mostly a collection of predefined filtering rules
//! along with the [`apply_rules`](crate::filters::apply_rules) function which can
//! apply a list of rules to a text.
//!
//! See the [`rules`](crate::rules) module for the lists of available filter rules.
//!
//! # Example
//! Generally you will want to combine several filter rules and then apply them to some text:
//! ```
//! use metadata_filter::rules::{remastered_filter_rules, trim_whitespace_filter_rules};
//! use metadata_filter::filters::apply_rules;
//!
//! let rules = [remastered_filter_rules(), trim_whitespace_filter_rules()].concat();
//! let filtered = apply_rules("Here Comes The Sun (Remastered)", &rules);
//!
//! assert_eq!(filtered, "Here Comes The Sun");
//! ```

pub mod filters;
pub mod rules;
