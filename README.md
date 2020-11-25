# metadata-filter

[![Crate](https://img.shields.io/crates/v/metadata-filter)](https://crates.io/crates/metadata-filter)
[![Documentation](https://docs.rs/metadata-filter/badge.svg)](https://docs.rs/metadata-filter)

A Rust library to clean up music metadata such as artist, album and song names.
This is a Rust port of the [metadata-filter](https://github.com/web-scrobbler/metadata-filter)
TypeScript library.

Currently, this library is mostly a collection of predefined filtering rules
along with the `apply_rules` function which can apply a list of rules to a text.
