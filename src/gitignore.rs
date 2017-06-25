extern crate regex;

use colored::*;
use std::process::exit;
use regex::RegexSet;
use nom::IResult;
use std::path::PathBuf;

/// Given a darcs boring file's contents, process it as a `RegexSet`. The second
/// argument is a file path, included so that we print nice errors.
pub fn darcs_contents_to_regex(file: &str, file_path: &PathBuf) -> RegexSet {

    let processed_vec: Vec<&str> = process_darcs_full(file);
    let processed_str: String = processed_vec.join("");
    let lines = processed_str.split_whitespace();

    let maybe_set = RegexSet::new(lines);
    if let Ok(s) = maybe_set {
        s
    } else {
        eprintln!(
            "{}: failed to parse darcs boring file at {:?}, ignoring",
            "Warning".yellow(),
            file_path
        );
        let empty: Vec<&str> = Vec::new();
        RegexSet::new(empty).expect("Error creating regex from empty vector")
    }

}

/// Given a `.gitignore` or `.ignore` file's contents, process it as a `RegexSet`. The second
/// argument is a file path, included so that we print nice errors.
pub fn file_contents_to_regex(file: &str, file_path: &PathBuf) -> RegexSet {
    let processed_vec: Vec<&str> = process_to_vector(file);
    let processed_str: String = processed_vec.join("");
    let lines = processed_str.split_whitespace();

    let maybe_set = RegexSet::new(lines);
    if let Ok(s) = maybe_set {
        s
    } else {
        eprintln!(
            "{}: failed to parse .gitignore/.ignore at {:?}, ignoring",
            "Warning".yellow(),
            file_path
        );
        let empty: Vec<&str> = Vec::new();
        RegexSet::new(empty).expect("Error creating regex from empty vector")
    }
}

fn process_to_vector(input: &str) -> Vec<&str> {
    match process(input) {
        IResult::Done(_, result) => result,
        _ => {
            eprintln!("{}: Failed to parse gitignore", "Error".red());
            exit(0xf001)
        }
    }
}

fn process_darcs_full(input: &str) -> Vec<&str> {
    match process_darcs(input) {
        IResult::Done(_, result) => result,
        _ => {
            eprintln!("{}: Failed to parse gitignore", "Error".red());
            exit(0xf001)
        }
    }
}

named!(process_darcs<&str, Vec<&str>>,
    do_parse!(
        opt!(first_line) >>
        r: many0!(darcs) >>
        (r)
    )
);

named!(process<&str, Vec<&str>>,
    do_parse!(
        opt!(first_line) >>
        r: many0!(options) >>
        (r)
    )
);

named!(darcs<&str, &str>,
    alt!(
        tag!("\n") |
        gitignore_comment |
        is_not!("\\#") |
        parse_backslash |
        parse_not_comment
    )
);

named!(options<&str, &str>,
    alt!(
        tag!("\n") |
        gitignore_comment |
        is_not!("*?.#") |
        parse_asterix |
        parse_period |
        parse_questionmark |
        parse_not_comment
    )
);

named!(parse_not_comment<&str, &str>,
    do_parse!(
        is_not!("\n") >>
        ("#")
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
        ("\n")
    )
);

named!(parse_period<&str, &str>,
    do_parse!(
        tag!(".") >>
        ("\\.")
    )
);

named!(parse_backslash<&str, &str>,
    do_parse!(
        val: alt!(
            do_parse!(tag!("\\_") >> ("_")) |
            do_parse!(tag!("\\") >> ("\\"))
            ) >>
        (val)
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
