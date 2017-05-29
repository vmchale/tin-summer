#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate colored;

pub mod types;
pub mod error;

use std::fs;
use std::path::PathBuf;
use regex::Regex;
use types::*;
use colored::*;

/// function to determine whether something is an artifact. 
///
/// Rules:
/// - if it`s included in the .gitignore and has a `.json` `.log` extension or something of the like,
/// it`s probably an artifact
/// - if it`s a `.a` or `.o` (or `.keter`, `.ll`, `.bc`, `.dyn\_o`) it`s probably an artifact
/// - if it`s a .sh file w/ interpreter it`s probably *not* an artifact
/// - elsewise, if it`s an executable *not* on the path it`s probaby an artifact (unix)
/// - if it`s .dll or .exe it`s probably an artifact (windows)
/// - if there`s a Cargo.toml + target/ that`s probably an artifact
/// - .stack-work is probably an artifact
/// - .log or .dvi in a folder w/ a .tex is probably 
/// - also, don`t forget .gitignore_global
#[cfg(not(os = "windows"))]
fn is_artifact(p: &PathBuf, re: Option<&Regex>) -> bool {
    let path_str = p.clone().into_os_string().into_string().expect("OS String invalid.");
    if let Some(r) = re {
        r.is_match(&path_str)
    }
    else {
        lazy_static! {
            static ref REGEX: Regex = 
                Regex::new(r".*?\.(a|o|ll|keter|bc|dyn_o|out|rlib|crate|min\.js|hi|dyn_hi|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o)$")
                .unwrap();
        }
        REGEX.is_match(&path_str)
    }
}

#[cfg(os = "windows")]
fn is_artifact(p: PathBuf, re: Option<&Regex>) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r".*?\.(exe|dll|ll|keter|bc|rlib|crate)").unwrap();
    }
    let path_str = &p.into_os_string().into_string().expect("OS String invalid.");
    if let Some(r) = re {
        r.is_match(path_str)
    }
    else {
        REGEX.is_match(path_str)
    }
}

pub fn read_files(in_paths: &PathBuf, depth: u8, min_bytes: Option<u64>, silent: bool) -> FileTree {
    let mut tree = FileTree::new();
    let mut total_size = FileSize::new(0);

    if let Ok(paths) = fs::read_dir(&in_paths) {
        for p in paths {
            let path = p.unwrap().path(); // TODO no unwraps; idk what this error would be though.

            // if this fails, it's probably because `path` is a symlink, so we ignore it.
            if let Ok(metadata) = fs::metadata(&path) {
                // append file size/name for a file
                if metadata.is_file() {
                    let file_size = FileSize::new(metadata.len());
                    if let Some(b) = min_bytes {
                        if file_size >= FileSize::new(b) {
                            tree.push(path.clone(), file_size, None, depth + 1);
                        }
                    }
                    else {
                        tree.push(path, file_size, None, depth + 1);
                    }
                    total_size.add(file_size);
                }

                // otherwise, go deeper
                else if metadata.is_dir() {
                    let mut subtree = read_files(&path, depth + 1, min_bytes, silent);
                    let dir_size = subtree.file_size;
                    if let Some(b) = min_bytes {
                        if dir_size >= FileSize::new(b) {
                            tree.push(path, dir_size, Some(&mut subtree), depth + 1);
                        }
                    }
                    else {
                        tree.push(path, dir_size, Some(&mut subtree), depth + 1);
                    }
                    total_size.add(dir_size);
                }
            }
            else if !silent {
                println!("{}: ignoring symlink at {}", "Warning".yellow(), path.display());
            }
        }
    }
    else if !silent {
        println!("{}: permission denied for directory: {}", "Warning".yellow(), &in_paths.display());
    }
    tree
}

pub fn read_artifacts(in_paths: &PathBuf, depth: u8, min_bytes: Option<u64>, artifact_regex: Option<&Regex>, excludes: Option<&Regex>, silent: bool) -> FileTree {
    let mut tree = FileTree::new();
    let mut total_size = FileSize::new(0);

    if let Ok(paths) = fs::read_dir(&in_paths) {
        for p in paths {
            let path = p.unwrap().path(); // TODO no unwraps; idk what this error would be though.
            let path_string = &path.clone().into_os_string().into_string().expect("OS String invalid.");
            let bool_loop = match excludes {
                Some(ex) => !ex.is_match(path_string),
                _ => true,
            };
            if bool_loop {
                // if this fails, it's probably because `path` is a symlink, so we ignore it.
                if let Ok(metadata) = fs::metadata(&path) {
                    // append file size/name for a file
                    if metadata.is_file() {
                        if is_artifact(&path, artifact_regex) {
                            let file_size = FileSize::new(metadata.len());
                            if let Some(b) = min_bytes {
                                if file_size >= FileSize::new(b) {
                                    tree.push(path, file_size, None, depth + 1);
                                }
                            }
                            else {
                                tree.push(path, file_size, None, depth + 1);
                            }
                            total_size.add(file_size);
                        }
                    }

                    // otherwise, go deeper
                    else if metadata.is_dir() {
                        let mut subtree = read_artifacts(&path, depth + 1, min_bytes, artifact_regex, excludes, silent);
                        let dir_size = subtree.file_size;
                        if let Some(b) = min_bytes {
                            if dir_size >= FileSize::new(b) {
                                tree.push(path, dir_size, Some(&mut subtree), depth + 1);
                            }
                        }
                        else { tree.push(path, dir_size, Some(&mut subtree), depth + 1); }
                        total_size.add(dir_size);
                        }
                }
                else if !silent { println!("{}: ignoring symlink at {}", "Warning".yellow(), path.display()); }
            }
        }
    }
    else if !silent {
        println!("{}: permission denied for directory: {}", "Warning".yellow(), &in_paths.display());
    }
    tree
}

pub fn read_files_regex(in_paths: &PathBuf, depth: u8, min_bytes: Option<u64>, regex: &Regex, silent: bool) -> FileTree {
    let mut tree = FileTree::new();
    let mut total_size = FileSize::new(0);

    if let Ok(paths) = fs::read_dir(&in_paths) {
        for p in paths {
            let path = p.unwrap().path(); // TODO no unwraps; idk what this error would be though.
            let path_string = &path.clone().into_os_string().into_string().expect("OS String invalid.");

            if !regex.is_match(path_string) {
                // if this fails, it's probably because `path` is a symlink, so we ignore it.
                if let Ok(metadata) = fs::metadata(&path) {
                    // append file size/name for a file
                    if metadata.is_file() {
                        let file_size = FileSize::new(metadata.len());
                        if let Some(b) = min_bytes {
                            if file_size >= FileSize::new(b) {
                                tree.push(path.clone(), file_size, None, depth + 1);
                            }
                        }
                        else {
                            tree.push(path, file_size, None, depth + 1);
                        }
                        total_size.add(file_size);
                    }

                    // otherwise, go deeper
                    else if metadata.is_dir() {
                        let mut subtree = read_files_regex(&path, depth + 1, min_bytes, regex, silent);
                        let dir_size = subtree.file_size;
                        if let Some(b) = min_bytes {
                            if dir_size >= FileSize::new(b) {
                                tree.push(path, dir_size, Some(&mut subtree), depth + 1);
                            }
                        }
                        else {
                            tree.push(path, dir_size, Some(&mut subtree), depth + 1);
                        }
                        total_size.add(dir_size);
                    }
                }
                else if !silent {
                    println!("{}: ignoring symlink at {}", "Warning".yellow(), path.display());
                }
            }
        }
    }
    else if !silent {
        println!("{}: permission denied for directory: {}", "Warning".yellow(), &in_paths.display());
    }
    tree
}
