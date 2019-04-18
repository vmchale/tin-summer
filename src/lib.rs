//! This is the library providing supporting functionality for the `sn` binary. The APIs here
//! aren't stable, but you may find useful documentation of how to use `sn`.

#![allow(unknown_lints)]
#![cfg_attr(feature = "cargo-clippy", allow(
    renamed_and_removed_lints,
    too_many_arguments,
    unreadable_literal,
))]

#[allow(unused_imports)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

extern crate colored;
extern crate regex;

pub mod cli_helpers;
pub mod error;
pub mod gitignore;
#[cfg(test)]
pub mod test;
pub mod types;
pub mod utils;
pub mod walk_parallel;

pub mod prelude {

    pub use cli_helpers::*;
    pub use error::*;
    pub use utils::*;
    pub use walk_parallel::*;

}
