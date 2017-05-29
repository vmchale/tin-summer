#[macro_use] extern crate clap;

extern crate libsniff;
extern crate colored;
extern crate regex;

use libsniff::*;
use std::path::PathBuf;
use clap::App;
use colored::*;
use regex::Regex;
use std::process;

#[allow(unknown_lints)] 
#[allow(cyclomatic_complexity)]
fn main() {
    // command-line parser
    #[cfg(feature = "english")]
    let yaml = load_yaml!("options-en.yml");
    #[cfg(feature = "français")]
    let yaml = load_yaml!("options-fr.yml");
    #[cfg(feature = "deutsch")]
    let yaml = load_yaml!("options-de.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // TODO parallel implementation for 'fat' and 'artifacts'
    if let Some(command) = matches.subcommand_matches("fat") {

        // set threshhold
        let min_bytes = if let Some(t) = command.value_of("threshhold") {
                match t {
                    "M" => 1048576,
                    "G" => 1073741824,
                    _ => { println!("{}: invalid threshhold; defaulting to M", "Warning".yellow()) ; 1048576 }
                }
            }
            else { 1048576 };

        // set depth
        let depth = 
            if let Some(n) = command.value_of("depth") {
                n.parse::<u8>().expect("Please enter a positive whole number")
            }
            else {
                2
            };

        // don't print warnings
        let silent = match matches.occurrences_of("v") {
            0 => true,
            _ => false,
        };

        // set regex for exclusions
        let regex = 
            if let Some(n) = command.value_of("excludes") {
                Some(n)
            }
            else {
                None
            };

        // set path to dir
        let path_read = command.value_of("dir");
        let init_dir = 
            if let Some(read) = path_read {
                let mut path_in = PathBuf::new();
                path_in.push(read);
                path_in
            }
            else {
                // default path is "./"
                PathBuf::from("./")
            };

        // decide what to print
        let v = if let Some(r) = regex {
                let re = if let Ok(rr) = Regex::new(r) {
                        rr
                    }
                    else if let Err(x) = Regex::new(r) {
                        println!("{}: Invalid regex:\n    {}", "Error".red(), x);
                        process::exit(0x0f01)
                    }
                    else {
                        process::exit(0x0f01)
                    };
                read_files_regex(&init_dir, 0, Some(min_bytes), &re, silent)
            }
            else {
                read_files(&init_dir, 0, Some(min_bytes), silent)
            };
        let mut v_filtered = v.filtered(depth);

        v_filtered.display_tree(init_dir);
    }

    else if let Some(command) = matches.subcommand_matches("artifacts") {

        // set threshhold
        let min_bytes = if let Some(t) = command.value_of("threshhold") {
                match t {
                    "M" => Some(1048576),
                    "G" => Some(1073741824),
                    _ => { println!("{}: invalid threshhold; defaulting to M", "Warning".yellow()) ; Some(1048576) }
                }
            }
            else { None };

        // set depth
        let depth = 
            if let Some(n) = command.value_of("depth") {
                n.parse::<u8>().expect("Please enter a positive whole number")
            }
            else {
                2
            };

        // don't print warnings
        let silent = match matches.occurrences_of("v") {
            0 => true,
            _ => false,
        };

        // set regex for artifacts
        let artifacts = 
            if let Some(n) = command.value_of("regex") {
                //let set = RegexSet::new(&[
                //    r"[a-z]+@[a-z]+\.(com|org|net)",
                //    r"[a-z]+\.(com|org|net)",
                //]).unwrap();
                Some(n)
            }
            else {
                None
            };

        // set path to dir
        let path_read = command.value_of("dir");
        let init_dir = 
            if let Some(read) = path_read {
                let mut path_in = PathBuf::new();
                path_in.push(read);
                path_in
            }
            else {
                // default path is "./"
                PathBuf::from("./")
            };

        // decide what to print
        let v = if let Some(r) = artifacts {
                let re = if let Ok(rr) = Regex::new(r) {
                        rr
                    }
                    else if let Err(x) = Regex::new(r) {
                        println!("{}: Invalid regex:\n    {}", "Error".red(), x);
                        process::exit(0x0f01)
                    }
                    else {
                        process::exit(0x0f01)
                    };
                read_artifacts(&init_dir, 0, min_bytes, Some(&re), silent)
            }
            else {
                read_artifacts(&init_dir, 0, min_bytes, None, silent)
            };
        let mut v_filtered = v.filtered(depth);

        v_filtered.display_tree(init_dir);
    }
    else if let Some(command) = matches.subcommand_matches("sort") {

        // set threshhold
        let min_bytes = if let Some(t) = command.value_of("threshhold") {
                match t {
                    "M" => Some(1048576),
                    "G" => Some(1073741824),
                    _ => { println!("{}: invalid threshhold; defaulting to M", "Warning".yellow()) ; Some(1048576) }
                }
            }
            else { None };

        // set number of things to fetch
        let num_int = 
            if let Some(num) = command.value_of("count") {
                num.parse::<usize>().expect("Please enter a positive whole number")
            }
            else {
                20
            };

        // set depth
        let depth = 
            if let Some(n) = command.value_of("depth") {
                n.parse::<u8>().expect("Please enter a positive whole number")
            }
            else {
                2
            };

        // don't print warnings
        let silent = match matches.occurrences_of("v") {
            0 => true,
            _ => false,
        };

        // set path to dir
        let path_read = command.value_of("dir");
        let init_dir = 
            if let Some(read) = path_read {
                let mut path_in = PathBuf::new();
                path_in.push(read);
                path_in
            }
            else {
                // default path is "./"
                PathBuf::from("./")
            };

        // set regex for exclusions
        let regex = 
            if let Some(n) = command.value_of("excludes") {
                Some(n)
            }
            else {
                None
            };

        let v = if let Some(r) = regex {
                let re = if let Ok(rr) = Regex::new(r) {
                        rr
                    }
                    else if let Err(x) = Regex::new(r) {
                        println!("{}: Invalid regex:\n    {}", "Error".red(), x);
                        process::exit(0x0f01)
                    }
                    else {
                        process::exit(0x0f01)
                    };
                read_files_regex(&init_dir, 0, min_bytes, &re, silent)
            }
            else {
                read_files(&init_dir, 0, min_bytes, silent)
            };
        let mut v_sorted = v.sort(Some(num_int), depth);

        v_sorted.display_tree(init_dir);
    }
    else {
        println!("{}: Command not recognized. Try 'sniff --help' if you're stuck.", "Error".red());
    }
}
