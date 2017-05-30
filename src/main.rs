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

    // find large files
    if let Some(command) = matches.subcommand_matches("fat") {

        // set threshhold
        let min_bytes = match command.value_of("threshhold") {
            Some(t) => match t {
                        "M" => 1048576,
                        "G" => 1073741824,
                        _ => { println!("{}: invalid threshhold; defaulting to M", "Warning".yellow()) ; 1048576 }
                        },
            _ => 1048576,
        };

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // don't print warnings
        let silent = command.is_present("silent");

        // set regex for exclusions
        let regex = command.value_of("excludes"); 

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = match regex {
                Some(r) => read_all(&init_dir, 0, Some(min_bytes), None, Some(&check_regex(r)), silent, false),
                _ => read_all(&init_dir, 0, Some(min_bytes), None, None, silent, false),
        };

        // filter by depth
        let mut v_filtered = v.filtered(depth);

        // display results
        v_filtered.display_tree(init_dir);
    }

    else if let Some(command) = matches.subcommand_matches("artifacts") {

        // set threshhold
        let min_bytes = match command.value_of("threshhold") {
            Some(t) => match t {
                        "M" => Some(1048576),
                        "G" => Some(1073741824),
                        _ => { println!("{}: invalid threshhold; defaulting to M", "Warning".yellow()) ; Some(1048576) }
                        },
            _ => None,
        };

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // set number of things to fetch for sort
        let num_int = get_num(command.value_of("count")); 

        // decide whether to sort
        let should_sort = command.is_present("sort");

        // don't print warnings
        let silent = command.is_present("silent");

        // set regex for artifacts
        let artifacts = command.value_of("regex"); 

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
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

    // sort entities by size
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

        // set number of things to fetch for sort
        let num_int = get_num(command.value_of("count")); 

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // decide whether to warnings
        let silent = command.is_present("silent");

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

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => read_all(&init_dir, 0, min_bytes, None, Some(&check_regex(r)), silent, false),
            _ => read_all(&init_dir, 0, min_bytes, None, None, silent, false),
        };

        // sort them
        let mut v_sorted = v.sort(Some(num_int), depth);

        // display sorted filenames
        v_sorted.display_tree(init_dir);
    }

    else {
        println!("{}: Command not recognized. Try 'sniff --help' if you're stuck.", "Error".red());
    }
}
