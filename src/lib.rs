//! This is the library providing supporting functionality for the `sn` binary. The APIs here
//! aren't stable, but you may find useful documentation of how to use `sn`.
#![allow(too_many_arguments)]
#![allow(unknown_lints)]
#![allow(useless_attribute)]
#![allow(unreadable_literal)]

#[allow(unused_imports)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate colored;

#[cfg(test)]
pub mod test;
pub mod types;
pub mod error;
pub mod cli_helpers;
pub mod gitignore;
pub mod utils;
pub mod walk_parallel;

pub mod prelude {

    pub use cli_helpers::*;
    pub use error::*;
    pub use utils::*;
    pub use walk_parallel::*;

}
