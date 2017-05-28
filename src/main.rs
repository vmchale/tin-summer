#[macro_use] extern crate clap;

extern crate libsniff;

use libsniff::*;
use std::path::PathBuf;
use clap::App;

fn main() {
    // command-line parser
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    if let Some(command) = matches.subcommand_matches("fat") {

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

        // decide whether to print ids
        let mut v = read_files(init_dir.clone(), 0);
        v.sort(Some(num_int), depth);

        v.display_tree(init_dir);
    }
    else {
        panic!("Only one current command accepted.");
    }
}
