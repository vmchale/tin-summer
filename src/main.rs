#[macro_use] extern crate clap;

extern crate libsniff;
extern crate colored;

use libsniff::*;
use std::path::PathBuf;
use clap::App;
use colored::*;

fn main() {
    // command-line parser
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

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
                Some(n.parse::<u8>().expect("Please enter a positive whole number"))
            }
            else {
                Some(2)
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
        let v = read_files(&init_dir, 0, Some(min_bytes));
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
                Some(n.parse::<u8>().expect("Please enter a positive whole number"))
            }
            else {
                Some(2)
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

        let v = read_files(&init_dir, 0, min_bytes);
        let mut v_sorted = v.sort(Some(num_int), depth);

        v_sorted.display_tree(init_dir);
    }
    else {
        println!("{}: Command not recognized. Try 'sniff --help' if you're stuck.", "Error".red());
    }
}
