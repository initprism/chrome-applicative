#![cfg_attr(feature = "nightly", feature(external_doc))]
#![deny(clippy::pedantic)]
#![warn(renamed_and_removed_lints)]
#![allow(
clippy::unknown_clippy_lints,
clippy::module_name_repetitions,
clippy::doc_markdown, // a number of false positives here
clippy::default_trait_access, // fails on output of derive_builder
clippy::needless_pass_by_value, // would stop us creating and passing in LaunchOptions to browser in one statement
clippy::unreadable_literal, // not really applicable for timestamps
clippy::too_many_lines,
clippy::type_repetition_in_bounds,
clippy::used_underscore_binding
)]

#[macro_use]
extern crate derive_builder;
extern crate log;

pub use browser::{
    tab::{element::Element, Tab},
    Browser, LaunchOptions, LaunchOptionsBuilder,
};

#[cfg(feature = "fetch")]
pub use browser::FetcherOptions;

pub mod browser;
pub mod protocol;
pub mod util;

#[cfg(feature = "nightly")]
#[doc(include = "../README.md")]
#[allow(dead_code)]
type _READMETEST = ();