use clap::Values;
use colored::*;
use error::*;
use nom::IResult;
use regex::Regex;
use std::path::PathBuf;
use std::process::exit;
use utils::get_processors;

/// Parse a string into a regular expression for the 'artifacts' subcommand. Adds ignores for
/// typical version control directories if none are present.
pub fn get_excludes(cli_excludes: Option<&str>) -> Regex {
    match cli_excludes {
        Some(s) => {
            let mut x = "(".to_string();
            x.push_str(s);
            x.push_str(r")|(\.git|\.pijul|_darcs|\.hg|\.gnupg)$");
            check_regex(&x)
        }
        _ => Regex::new(r"(\.git|\.pijul|_darcs|\.hg|\.gnupg)$").unwrap(), // ok because static
    }
}

pub fn get_depth(depth_from_cli: Option<&str>) -> u8 {
    if let Some(n) = depth_from_cli {
        if let Ok(n) = n.parse::<u8>() {
            n
        } else {
            eprintln!("{}", Internal::ParseNum);
            exit(0x0f01);
        }
    } else {
        2
    }
}

pub fn get_num(num_from_cli: Option<&str>) -> usize {
    if let Some(num) = num_from_cli {
        if let Ok(n) = num.parse::<usize>() {
            n
        } else {
            eprintln!("{}", Internal::ParseNum);
            exit(0x0f01);
        }
    } else {
        8
    }
}

/// If the user has supplied a string, parse it, otherwise, read the number of processors.
pub fn get_threads(num_from_cli: Option<&str>) -> usize {
    match num_from_cli {
        Some(num) => {
            if let Ok(n) = num.parse::<usize>() {
                n
            } else {
                eprintln!("{}", Internal::ParseNum);
                exit(0x0f01);
            }
        }
        _ => get_processors(),
    }
}

pub fn get_dirs(paths_from_cli: Option<Values>) -> Vec<PathBuf> {
    if let Some(read) = paths_from_cli {
        read.map(PathBuf::from).collect()
    } else {
        let mut v = Vec::new();
        v.push(PathBuf::from("."));
        v
    }
}

pub fn get_dir(path_from_cli: Option<&str>) -> PathBuf {
    match path_from_cli {
        Some(read) => PathBuf::from(read),
        _ => PathBuf::from("."),
    }
}

/// Parse a threshold from a command-line flag.
///
/// # Examples
///
/// ```
/// use liboskar::prelude::*;
///
/// let threshold_string = Some("31M");
/// assert_eq!(threshold(threshold_string), Some(32505856))
/// ```
pub fn threshold(s: Option<&str>) -> Option<u64> {
    s.map(pre_threshold)
}

fn pre_threshold(t_from_cli: &str) -> u64 {
    match get_threshold(t_from_cli.as_bytes()) {
        IResult::Done(_, n) => n,
        _ => {
            eprintln!(
                "{}: failed to parse threshold. defaulting to 1M",
                "Warning".yellow()
            );
            1048576 // 1 MB
        }
    }
}

fn to_u64(nums: Vec<char>, size_tag: &[u8]) -> u64 {
    let pre: String = nums.into_iter().collect();
    let n = if let Ok(n) = pre.parse::<u64>() {
        n
    } else {
        eprintln!("{}", Internal::ParseNum);
        exit(0x0f01);
    };

    match size_tag {
        b"G" | b"g" => n * 1073741824,
        b"M" | b"m" => n * 1048576,
        b"k" | b"K" => n * 1024,
        b"b" | b"B" => n,
        _ => exit(0x0f01),
    }
}

named!(digit_char<&[u8], char>,
    alt!(
        char!('1') |
        char!('2') |
        char!('3') |
        char!('4') |
        char!('5') |
        char!('6') |
        char!('7') |
        char!('8') |
        char!('9') |
        char!('0')
    )
);

named!(get_threshold<&[u8],u64>,
    do_parse!(
        nums:     many1!(digit_char) >>
        size_tag: alt!(tag!("M") |
                       tag!("G") |
                       tag!("k") |
                       tag!("b") |
                       tag!("B") |
                       tag!("K") |
                       tag!("g") |
                       tag!("m")) >>
        (to_u64(nums, size_tag))
    )
);
