extern crate num_cpus;

use std::fs::File;
use std::io::prelude::*;
use regex::RegexSet;
use gitignore::*;
use std::path::PathBuf;
use self::num_cpus::get;

/// Gather the information from `.gitignore`, `.ignore`, and darcs `boring` files in a given
/// directory, and assemble a 'RegexSet' from it.
pub fn mk_ignores(in_paths: &PathBuf, maybe_gitignore: &Option<RegexSet>) -> Option<RegexSet> {

    if let Some(ref gitignore) = *maybe_gitignore {
        Some(gitignore.to_owned())
    } else if let (gitignore_path, Ok(mut file)) =
        {
            let mut gitignore_path = in_paths.clone();
            gitignore_path.push(".gitignore");
            (gitignore_path.clone(), File::open(gitignore_path))
        } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed.");
        Some(file_contents_to_regex(&contents, &gitignore_path))
    } else if let (darcs_path, Ok(mut file)) =
        {
            let mut darcs_path = in_paths.clone();
            darcs_path.push("_darcs/prefs/boring");
            (darcs_path.clone(), File::open(darcs_path))
        } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed.");
        Some(darcs_contents_to_regex(&contents, &darcs_path))
    } else if let (ignore_path, Ok(mut file)) =
        {
            let mut ignore_path = in_paths.clone();
            ignore_path.push(".ignore");
            (ignore_path.clone(), File::open(ignore_path.clone()))
        } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed.");
        Some(file_contents_to_regex(&contents, &ignore_path))
    } else {
        None
    }
}

/// Helper function to get the number of CPUs. We subtract 1, because the main thread that's doing
/// the spawning counts as one OS thread.
pub fn get_processors() -> usize {
    let n = get();
    if n > 1 {
        n - 1
    } else {
        n
    }
}
