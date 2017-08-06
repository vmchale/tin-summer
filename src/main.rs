#[macro_use]
extern crate clap;

extern crate liboskar;
extern crate regex;
extern crate colored;

use colored::*;
use liboskar::prelude::*;
use clap::{App, AppSettings};
use std::env;
use std::path::PathBuf;

#[allow(unknown_lints)]
#[allow(cyclomatic_complexity)]
fn main() {

    // command-line parser
    #[cfg(feature = "english")]
    let yaml = load_yaml!("cli/options-en.yml");
    #[cfg(feature = "francais")]
    let yaml = load_yaml!("cli/options-fr.yml");
    #[cfg(feature = "deutsch")]
    let yaml = load_yaml!("cli/options-de.yml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .set_term_width(90)
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    if let Some(command) = matches.subcommand_matches("clean") {

        // check that we aren't in the home dir
        let home_dir_str = match env::var("HOME") {
            Ok(val) => val,
            _ => "/home".to_string(),
        };

        let home_dir = PathBuf::from(home_dir_str);

        let regex = command.value_of("excludes").map(|r| check_regex(r));

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        let force = command.is_present("force");

        for dir in dirs {
            if (dir != home_dir) && !force {
                clean_project_dirs(dir, regex.clone());
            }
            else {
                eprintln!("{}: not cleaning directory '{}', as it is your home directory. To clean your home directory, rerun with --force.", "Warning".yellow(), dir.display())
            }
        }
    }
        
    // test stuff
    else if let Some(command) = matches.subcommand_matches("parallel") {

        // set flag to print everything
        let print_all = command.is_present("all");

        // set whether to print files too
        let print_files = command.is_present("files");

        // get the number of processors to be used
        let nproc = get_threads(command.value_of("threads"));

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        let regex = command.value_of("excludes").map(|r| check_regex(r));

        for dir in dirs {

            let mut w = Walk::new(dir, nproc);
            if let Some(b) = min_bytes {
                w.set_threshold(b);
            }
            if !print_all {
                let depth = get_depth(command.value_of("depth"));
                w.set_depth(depth);
            } else if command.is_present("depth") {
                eprintln!(
                    "{}: flag --all is not compatible with --depth",
                    "Warning".yellow()
                );
            }
            if print_files {
                w.with_files();
            }

            if let Some(e) = regex.clone() {
                w.set_regex(e);
            }

            print_parallel(w);
        }

    }
    // find large files
    else if let Some(command) = matches.subcommand_matches("fat") {

        // set threshold
        let min_bytes = match threshold(command.value_of("threshold")) {
            Some(t) => t,
            _ => 31457280, // 30 MB
        };

        // set depth
        let depth = if !command.is_present("all") {
            Some(get_depth(command.value_of("depth")))
        } else if command.is_present("depth") {
            eprintln!(
                "{}: flag --all is not compatible with --depth",
                "Warning".yellow()
            );
            None
        } else {
            None
        };

        // don't print warnings
        //let silent = command.is_present("silent");

        // set whether to print files too
        let print_files = command.is_present("files");

        // set regex for exclusions
        let regex = command.value_of("excludes");

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        for dir in dirs {

            // get relevant filenames &c.
            let v = match regex {
                Some(r) => read_all(&dir, 0, depth, Some(&check_regex(r)), &None, false),
                _ => read_all(&dir, 0, depth, None, &None, false),
            };

            // filter by depth
            let mut v_filtered = v.filtered(Some(min_bytes), !print_files, depth);

            // display results
            v_filtered.display_tree(dir);
        }
    }
    // find large files
    else if let Some(command) = matches.subcommand_matches("directories") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = if !command.is_present("all") {
            Some(get_depth(command.value_of("depth")))
        } else if command.is_present("depth") {
            eprintln!(
                "{}: flag --all is not compatible with --depth",
                "Warning".yellow()
            );
            None
        } else {
            None
        };

        // set regex for exclusions
        let regex = command.value_of("excludes");

        // set whether to print files too
        let print_files = command.is_present("files");

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        for dir in dirs {

            // get relevant filenames &c.
            let v = match regex {
                Some(r) => read_all(&dir, 0, depth, Some(&check_regex(r)), &None, false),
                _ => read_all_fast(&dir, 0, depth),
            };


            // filter by depth
            let mut v_filtered = v.filtered(min_bytes, !print_files, depth);

            // display results
            v_filtered.display_tree(dir);
        }

    } else if let Some(command) = matches.subcommand_matches("files") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = if !command.is_present("all") {
            Some(get_depth(command.value_of("depth")))
        } else if command.is_present("depth") {
            eprintln!(
                "{}: flag --all is not compatible with --depth",
                "Warning".yellow()
            );
            None
        } else {
            None
        };

        // set regex for exclusions
        let regex = command.value_of("excludes");

        // set whether to print files too
        let print_files = true;

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        for dir in dirs {

            // get relevant filenames &c.
            let v = match regex {
                Some(r) => read_all(&dir, 0, depth, Some(&check_regex(r)), &None, false),
                _ => read_all(&dir, 0, depth, None, &None, false),
            };


            // filter by depth
            let mut v_filtered = v.filtered(min_bytes, !print_files, depth);

            // display results
            v_filtered.display_tree(dir);
        }

    } else if let Some(command) = matches.subcommand_matches("artifacts") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = if !command.is_present("all") {
            Some(get_depth(command.value_of("depth")))
        } else if command.is_present("depth") {
            eprintln!(
                "{}: flag --all is not compatible with --depth",
                "Warning".yellow()
            );
            None
        } else {
            None
        };

        // set number to print out
        let num_int = if command.is_present("count") {
            Some(get_num(command.value_of("count")))
        } else {
            None
        };

        // decide whether to sort
        let should_sort = command.is_present("sort");

        // set whether to print files too
        let print_files = command.is_present("files");

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        for dir in dirs {

            // get relevant filenames &c.
            let excludes = get_excludes(command.value_of("excludes"));
            let v = read_all(&dir, 0, depth, Some(&excludes), &None, true);

            let mut v_processed = if should_sort {
                v.sort(num_int, min_bytes, !print_files, depth)
            } else {
                v.filtered(min_bytes, !print_files, depth)
            };

            v_processed.display_tree(dir);
        }
    }
    // sort entities by size
    else if let Some(command) = matches.subcommand_matches("sort") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = if !command.is_present("all") {
            Some(get_depth(command.value_of("depth")))
        } else if command.is_present("depth") {
            eprintln!(
                "{}: flag --all is not compatible with --depth",
                "Warning".yellow()
            );
            None
        } else {
            None
        };

        // set number to print out
        let num_int = if command.is_present("count") {
            Some(get_num(command.value_of("count")))
        } else {
            None
        };

        // set whether to print files too
        let print_files = command.is_present("files");

        // set path to dirs
        let dirs = get_dirs(command.values_of("dir"));

        for dir in dirs {

            // set regex for exclusions
            let regex = if let Some(n) = command.value_of("excludes") {
                Some(n)
            } else {
                None
            };

            // get relevant filenames &c.
            let v = match regex {
                Some(r) => read_all(&dir, 0, depth, Some(&check_regex(r)), &None, false),
                _ => read_all(&dir, 0, depth, None, &None, false),
            };

            // sort them
            let mut v_sorted = v.sort(num_int, min_bytes, !print_files, depth);

            // display sorted filenames
            v_sorted.display_tree(dir);
        }
    }
}
