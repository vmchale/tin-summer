#[macro_use] extern crate clap;

extern crate libsniff;
extern crate colored;
extern crate regex;

use libsniff::*;
use libsniff::cli_helpers::*;
use libsniff::error::check_regex;
use clap::App;
use colored::*;

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

    // TODO parallel implementation for 'fat' and 'artifacts'?
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
        let silent = match matches.occurrences_of("silent") {
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
        let init_dir = get_dir(command.value_of("dir"));

        // decide what to print
        let v = match regex {
                Some(r) => read_all(&init_dir, 0, Some(min_bytes), None, Some(&check_regex(r)), silent, false),
                _ => read_all(&init_dir, 0, Some(min_bytes), None, None, silent, false),
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

        // set number of things to fetch for sort
        let num_int = 
            if let Some(num) = command.value_of("count") {
                num.parse::<usize>().expect("Please enter a positive whole number")
            }
            else {
                20
            };

        // decide whether to sort
        let should_sort = match matches.occurrences_of("sort") {
            0 => false,
            _ => true,
        };

        // don't print warnings
        let silent = match matches.occurrences_of("silent") {
            0 => true,
            _ => false,
        };

        // set regex for artifacts
        let artifacts = 
            if let Some(n) = command.value_of("regex") {
                Some(n)
            }
            else {
                None
            };

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // decide what to print
        let v = if let Some(r) = artifacts {
                let re = check_regex(r);
                match command.value_of("excludes") {
                    Some(ex) => read_all(&init_dir, 0, min_bytes, Some(&re), Some(&check_regex(ex)), silent, true),
                    _ => read_all(&init_dir, 0, min_bytes, Some(&re), None, silent, true),
                }
            }
            else {
                match command.value_of("excludes") {
                    Some(ex) => read_all(&init_dir, 0, min_bytes, None, Some(&check_regex(ex)), silent, true),
                    _ => read_all(&init_dir, 0, min_bytes, None, None, silent, true),
                }
            };

        let mut v_processed = if should_sort {
                v.sort(Some(num_int), depth)
            }
            else {
                v.filtered(depth)
            };

        v_processed.display_tree(init_dir);
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
        let silent = match matches.occurrences_of("silent") {
            0 => true,
            _ => false,
        };

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // set regex for exclusions
        let regex = 
            if let Some(n) = command.value_of("excludes") {
                Some(n)
            }
            else {
                None
            };

        let v = match regex {
            Some(r) => read_all(&init_dir, 0, min_bytes, None, Some(&check_regex(r)), silent, false),
            _ => read_all(&init_dir, 0, min_bytes, None, None, silent, false),
        };
        let mut v_sorted = v.sort(Some(num_int), depth);

        v_sorted.display_tree(init_dir);
    }
    else {
        println!("{}: Command not recognized. Try 'sniff --help' if you're stuck.", "Error".red());
    }
}
