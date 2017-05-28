extern crate pad;
extern crate regex;
extern crate colored;

pub mod types;
pub mod parser;

use std::fs;
use std::path::PathBuf;
use regex::Regex;
use types::*;
use colored::*;

fn is_artifact(p: PathBuf, re: Option<Regex>) -> bool {
    let regex = if let Some(r) = re { r } 
        else { Regex::new(r"^\S+.a").unwrap() };
    let path_str = &p.into_os_string().into_string().expect("OS String invalid.");
    regex.is_match(path_str)
}

pub fn read_files(in_paths: &PathBuf, depth: u8, min_bytes: Option<u64>) -> FileTree {
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
                    let mut subtree = read_files(&path, depth + 1, min_bytes);
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
            else {
                println!("{}: ignoring symlink at {}", "Warning".yellow(), path.display());
            }
        }
    }
    else {
        println!("{}: permission denied for directory: {}", "Warning".yellow(), &in_paths.display());
    }
    tree
}
