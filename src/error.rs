extern crate regex;
extern crate colored;

use regex::Regex;
use std::process;
use colored::*;

pub fn check_regex(re: &str) -> Regex {
    if let Ok(r) = Regex::new(re) {
        r
    } else if let Err(x) = Regex::new(re) {
        println!("{}: Invalid regex:\n    {}", "Error".red(), x);
        process::exit(0x0f01)
    } else {
        process::exit(0x0f01)
    }
}
