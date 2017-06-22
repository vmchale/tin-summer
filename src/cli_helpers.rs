extern crate regex;

use colored::*;
use nom::IResult;
use std::process::exit;
use std::path::PathBuf;
use regex::Regex;
use error::check_regex;
use utils::get_processors;

pub fn get_excludes(cli_excludes: Option<&str>) -> Regex {
    match cli_excludes {
        Some(s) => {
            let mut x = "(".to_string();
            x.push_str(s);
            x.push_str(r")|(\.git|\.pijul|_darcs|\.hg)$");
            check_regex(&x)
        }
        _ => Regex::new(r"(\.git|\.pijul|_darcs|\.hg)$").unwrap(),
    }
}

pub fn get_depth(depth_from_cli: Option<&str>) -> u8 {
    if let Some(n) = depth_from_cli {
        n.parse::<u8>()
            .expect("Please enter a positive whole number")
    } else {
        2
    }
}

pub fn get_num(num_from_cli: Option<&str>) -> usize {
    if let Some(num) = num_from_cli {
        num.parse::<usize>()
            .expect("Please enter a positive whole number")
    } else {
        8
    }
}

pub fn get_threads(num_from_cli: Option<&str>) -> usize {
    match num_from_cli {
        Some(num) =>num.parse::<usize>()
            .expect("Please enter a positive whole number"),
        _ => get_processors()
    }
}

pub fn get_dir(path_from_cli: Option<&str>) -> PathBuf {
    match path_from_cli {
        Some(read) => PathBuf::from(read),
        _ => PathBuf::from("."),
    }
}

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
            1048576
        }
    }
}

fn to_u64(nums: Vec<char>, size_tag: &[u8]) -> u64 {

    let pre: String = nums.into_iter().collect();
    let n = pre.parse::<u64>()
        .expect("Error parsing integer at cli_helpers.rs:58");

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
