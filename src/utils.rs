use colored::*;
use std::process::Command;

pub fn get_processors() -> usize {
    let nproc_output = Command::new("nproc")
                     .output();
    let out_string = match nproc_output {
        Ok(x) => String::from_utf8(x.stdout.into_iter().take_while(|c| *c != b'\n').collect()).expect("invalid unicode"),
        Err(_) => { eprintln!("{}: Could not detect number of processors.", "Warning".yellow()) ; "0".to_string() }, // can fail on e.g. windows
    };
    out_string.parse::<usize>().expect("nproc failed to return a ") - 1
}
