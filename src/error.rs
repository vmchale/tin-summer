extern crate regex;
extern crate colored;

use regex::Regex;
use std::process;
use colored::*;

/// Check that the user-supplied regex is valid. If not, fail gracefully.
pub fn check_regex(re: &str) -> Regex {
    if let Ok(r) = Regex::new(re) {
        r
    } else if let Err(x) = Regex::new(re) {
        eprintln!("{}: Invalid regex:\n    {}", "Error".red(), x);
        process::exit(0x0f01)
    } else {
        process::exit(0x0f01)
    }
}
