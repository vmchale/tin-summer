use regex::Regex;
use std::process;
use colored::*;
use std::fmt;

#[derive(Debug)]
pub enum Internal {
    ParseNum,
    ParseIgnore,
    GetPath,
    DirPermissions,
    NotDirectory,
    PathDoesNotExist,
    IoError,
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            &Internal::IoError => {
                write!(
                    f,
                    "{}: IO error when trying to read directory contents",
                    "Error".red()
                )
            }
            &Internal::ParseNum => {
                write!(
                    f,
                    "{}: Please enter a positive whole number.",
                    "Error".red()
                )
            }
            _ => write!(f, "other error"),
        }
    }
}

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
