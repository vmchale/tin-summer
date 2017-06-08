extern crate regex;

use colored::*;
use std::process::exit;
use regex::RegexSet;
use nom::IResult;
use std::path::PathBuf;

pub fn file_contents_to_regex(file: &str, file_path: &PathBuf) -> RegexSet {
    let processed_vec: Vec<&str> = process_to_vector(file);
    let processed_str: String = processed_vec.join("");
    let lines = processed_str.split_whitespace();

    debugln!("{:?}", lines.clone().collect::<Vec<&str>>());
   
    let maybe_set = RegexSet::new(lines);
    if let Ok(s) = maybe_set {
        s
    }
    else {
        eprintln!("{}: failed to parse .gitignore at {:?}, ignoring", "Warning".yellow(), file_path);
        let empty: Vec<&str> = Vec::new();
        RegexSet::new(empty)
            .expect("Error creating regex from empty vector")
    }
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
        is_not!("*?.") |
        parse_asterix |
        parse_period |
        parse_questionmark
    )
);

named!(first_line<&str, &str>,
    do_parse!(
        tag!("#") >>
        is_not!("\n") >>
        tag!("\n") >>
        ("\n")
    )
);

named!(gitignore_comment<&str, &str>,
    do_parse!(
        tag!("\n#") >>
        is_not!("\n") >>
        tag!("\n") >>
        ("\n")
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
        opt!(do_parse!(tag!("\n") >> eof!() >> (""))) >>
        (".*")
    )
);

named!(parse_questionmark<&str, &str>,
    do_parse!(
        tag!("?") >>
        opt!(do_parse!(tag!("\n") >> eof!() >> (""))) >>
        (".")
    )
);
