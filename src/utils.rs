extern crate num_cpus;

use self::num_cpus::get;
use gitignore::*;
use regex::RegexSet;
use std::fs::File;
use std::fs::Metadata;
use std::io::prelude::*;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris"
))]
use std::os::unix::fs::MetadataExt;

#[cfg(target_os = "linux")]
pub fn size(m: &Metadata, blocks: bool) -> u64 {
    if blocks {
        m.st_blocks() * 512
    } else {
        m.len()
    }
}

#[cfg(any(target_os = "windows", target_os = "redox"))]
pub fn size(m: &Metadata, _: bool) -> u64 {
    m.len()
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "solaris"
))]
pub fn size(m: &Metadata, blocks: bool) -> u64 {
    if blocks {
        m.blocks() * 512 // idk if this is correct on bsd
    } else {
        m.len()
    }
}

/// Gather the information from `.gitignore`, `.ignore`, and darcs `boring` files in a given
/// directory, and assemble a `RegexSet` from it.
pub fn mk_ignores(in_paths: &PathBuf, maybe_ignore: &Option<RegexSet>) -> Option<RegexSet> {
    if let Some(ref ignore) = *maybe_ignore {
        Some(ignore.to_owned())
    } else if let (ignore_path, Ok(mut file)) = {
        let mut ignore_path = in_paths.clone();
        ignore_path.push(".ignore");
        (ignore_path.clone(), File::open(ignore_path.clone()))
    } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed."); // ok because we check that the file exists
        Some(file_contents_to_regex(&contents, &ignore_path))
    } else if let (gitignore_path, Ok(mut file)) = {
        let mut gitignore_path = in_paths.clone();
        gitignore_path.push(".gitignore");
        (gitignore_path.clone(), File::open(gitignore_path))
    } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed."); // ok because we check that the file exists
        Some(file_contents_to_regex(&contents, &gitignore_path))
    } else if let (darcs_path, Ok(mut file)) = {
        let mut darcs_path = in_paths.clone();
        darcs_path.push("_darcs/prefs/boring");
        (darcs_path.clone(), File::open(darcs_path))
    } {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("File read failed."); // ok because we check that the file exists
        Some(darcs_contents_to_regex(&contents, &darcs_path))
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
