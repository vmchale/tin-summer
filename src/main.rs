#[macro_use]
extern crate clap;

extern crate liboskar;
extern crate colored;
extern crate regex;

use liboskar::prelude::*;
use clap::App;
use colored::*;

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
        .get_matches();

    // test stuff
    if let Some(command) = matches.subcommand_matches("parallel") {

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get the number of processors to be used
        let nproc = get_threads(command.value_of("threads"));

        let w = Walk::new(init_dir, nproc);
        print_parallel(w);

    }
    // find large files
    else if let Some(command) = matches.subcommand_matches("fat") {

        // set threshold
        let min_bytes = match threshold(command.value_of("threshold")) {
            Some(t) => t,
            _ => 1048576,
        };

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // don't print warnings
        //let silent = command.is_present("silent");

        // don't print warnings
        let print_files = command.is_present("files");

        // set regex for exclusions
        let regex = command.value_of("excludes");

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => {
                read_all(
                    &init_dir,
                    0,
                    Some(depth),
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, Some(depth), None, None, &None, false, false),
        };

        // filter by depth
        let mut v_filtered = v.filtered(Some(min_bytes), !print_files);

        // display results
        v_filtered.display_tree(init_dir);
    }
    // find large files
    else if let Some(command) = matches.subcommand_matches("all") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // set regex for exclusions
        let regex = command.value_of("excludes");

        // don't print warnings
        let print_files = command.is_present("files");

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => {
                read_all(
                    &init_dir,
                    0,
                    Some(depth),
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, Some(depth), None, None, &None, false, false),
        };


        // filter by depth
        let mut v_filtered = v.filtered(min_bytes, !print_files);

        // display results
        v_filtered.display_tree(init_dir);

    } else if let Some(command) = matches.subcommand_matches("artifacts") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // set number of things to fetch for sort
        let num_int = get_num(command.value_of("count"));

        // decide whether to sort
        let should_sort = command.is_present("sort");

        // don't print warnings
        let print_files = command.is_present("files");

        // set regex for artifacts
        let artifacts = command.value_of("regex");

        // decide whether to use gitignore information
        let no_gitignore = command.is_present("gitignore");

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = if let Some(r) = artifacts {
            let re = check_regex(r);
            let excludes = get_excludes(command.value_of("excludes"));
            read_all(
                &init_dir,
                0,
                Some(depth),
                Some(&re),
                Some(&excludes),
                &None,
                !no_gitignore,
                true,
            )
        } else {
            let excludes = get_excludes(command.value_of("excludes"));
            read_all(
                &init_dir,
                0,
                Some(depth),
                None,
                Some(&excludes),
                &None,
                !no_gitignore,
                true,
            )
        };

        let mut v_processed = if should_sort {
            v.sort(Some(num_int), min_bytes, !print_files)
        } else {
            v.filtered(min_bytes, !print_files)
        };

        v_processed.display_tree(init_dir);
    }
    // sort entities by size
    else if let Some(command) = matches.subcommand_matches("sort") {

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        // set number of things to fetch for sort
        let num_int = get_num(command.value_of("count"));

        // set depth
        let depth = get_depth(command.value_of("depth"));

        // don't print warnings
        let print_files = command.is_present("files");

        // set number of things to fetch for sort
        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // set regex for exclusions
        let regex = if let Some(n) = command.value_of("excludes") {
            Some(n)
        } else {
            None
        };

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => {
                read_all(
                    &init_dir,
                    0,
                    Some(depth),
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, Some(depth), None, None, &None, false, false),
        };

        // sort them
        let mut v_sorted = v.sort(Some(num_int), min_bytes, !print_files);

        // display sorted filenames
        v_sorted.display_tree(init_dir);
    } else {
        eprintln!(
            "{}: Command not recognized. Try 'sniff --help' if you're stuck.",
            "Error".red()
        );
    }
}
