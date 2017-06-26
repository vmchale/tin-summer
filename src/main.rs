#[macro_use]
extern crate clap;

extern crate liboskar;
extern crate regex;
extern crate colored;

use colored::*;
use liboskar::prelude::*;
use clap::{App, AppSettings};

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

    // test stuff
    if let Some(command) = matches.subcommand_matches("parallel") {

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // set flag to print everything
        let print_all = command.is_present("all");

        // set whether to print files too
        let print_files = command.is_present("files");

        // get the number of processors to be used
        let nproc = get_threads(command.value_of("threads"));

        // set threshold
        let min_bytes = threshold(command.value_of("threshold"));

        let mut w = Walk::new(init_dir, nproc);
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

        if let Some(e) = command.value_of("excludes") {
            w.set_regex(check_regex(e));
        }

        print_parallel(w);

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

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => {
                read_all(
                    &init_dir,
                    0,
                    depth,
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, depth, None, None, &None, false, false),
        };

        // filter by depth
        let mut v_filtered = v.filtered(Some(min_bytes), !print_files, depth);

        // display results
        v_filtered.display_tree(init_dir);
    }
    // find large files
    else if let Some(command) = matches.subcommand_matches("all") {

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

        // set path to dir
        let init_dir = get_dir(command.value_of("dir"));

        // get relevant filenames &c.
        let v = match regex {
            Some(r) => {
                read_all(
                    &init_dir,
                    0,
                    depth,
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, depth, None, None, &None, false, false),
        };


        // filter by depth
        let mut v_filtered = v.filtered(min_bytes, !print_files, depth);

        // display results
        v_filtered.display_tree(init_dir);

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

        // set number of things to fetch for sort
        let num_int = get_num(command.value_of("count"));

        // decide whether to sort
        let should_sort = command.is_present("sort");

        // set whether to print files too
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
                depth,
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
                depth,
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
            v.filtered(min_bytes, !print_files, depth)
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

        // set whether to print files too
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
                    depth,
                    None,
                    Some(&check_regex(r)),
                    &None,
                    false,
                    false,
                )
            }
            _ => read_all(&init_dir, 0, depth, None, None, &None, false, false),
        };

        // sort them
        let mut v_sorted = v.sort(Some(num_int), min_bytes, !print_files);

        // display sorted filenames
        v_sorted.display_tree(init_dir);
    }
}
