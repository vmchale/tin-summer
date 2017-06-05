extern crate regex;

use colored::*;
use std::process::exit;
use regex::RegexSet;
use nom::IResult;

pub fn file_contents_to_regex(file: &str) -> RegexSet {
    let processed_vec: Vec<&str> = process_to_vector(file);
    let processed_str: String = processed_vec.join("");
    let lines = processed_str.split_whitespace();
    RegexSet::new(lines)
        .expect("Error parsing .gitignore") // TODO nicer error message here
}

pub fn process_to_vector(input: &str) -> Vec<&str> {
    match process(input) {
        IResult::Done(_, result) => result,
        _ => { eprintln!("{}: Failed to parse gitignore", "Error".red()) ; exit(0xf001) }
    }
}

named!(process<&str, Vec<&str>>,
    do_parse!(
        opt!(first_line) >>
        r: many0!(options) >>
        (r)
    )
);

named!(options<&str, &str>,
    alt!(
        gitignore_comment |
        is_not!("*?.\n") |
        parse_period |
        parse_asterix |
        parse_questionmark
    )
);

named!(first_line<&str, &str>,
    do_parse!(
        tag!("#") >>
        is_not!("\n") >>
        ("")
    )
);


named!(gitignore_comment<&str, &str>,
    do_parse!(
        tag!("\n#") >>
        is_not!("\n") >>
        ("")
    )
);

named!(parse_period<&str, &str>,
    do_parse!(
        tag!(".") >>
        ("\\.")
    )
);

named!(parse_asterix<&str, &str>,
    do_parse!(
        tag!("*") >>
        (".*")
    )
);

named!(parse_questionmark<&str, &str>,
    do_parse!(
        tag!("?") >>
        (".")
    )
);
