#![allow(dead_code)]

extern crate crossbeam;

use std::fs;
use self::crossbeam::sync::chase_lev;
use regex::{RegexSet, Regex};
use std::path::{PathBuf};
use colored::*;
use std::process::exit;
use std::thread;
use std::fs::DirEntry;

pub struct Walk {
    path: PathBuf,
    gitignore: Option<RegexSet>,
    hgignore: Option<RegexSet>,
    darcs_boring: Option<RegexSet>,
    excludes: Option<Regex>,
    max_depth: Option<u8>,
    threshold: Option<u64>,
}

impl Walk {

    pub fn new(p: PathBuf) -> Walk {
        Walk { path: p, gitignore: None, hgignore: None, darcs_boring: None, excludes: None, max_depth: None, threshold: None }
    }

    pub fn push_subdir(path: PathBuf, ref mut worker: &mut chase_lev::Worker<DirEntry>) {
        
        let in_paths = &path;
        
        // fill up queue + print out files
        if let Ok(paths) = fs::read_dir(in_paths) {

            // iterate over all the entries in the directory
            for p in paths {
                let val = match p {
                    Ok(x) => x,
                    _ => { eprintln!("{}: path error at {:?}.", "Error".red(), p) ; exit(0x0001) },
                };
                match val.file_type() { // ideally, we'd push *directories* instead, and also stream this stuff!
                    // basically, we'd like to push shallow directories only?
                    Ok(t) => { if t.is_file() { worker.push(val); } else if t.is_dir() { let mut new_path = path.to_owned() ; new_path.push(val.file_name()) ; Walk::push_subdir(new_path, worker) } },
                    _ => { eprintln!("{}: could not determine file type for: {}", "Warning".yellow(), val.file_name().to_str().unwrap()) }
                }
            }
        }

        // if we can't read the directory contents, figure out why
        // 1: check the path exists
        else if !in_paths.exists() {
            eprintln!(
                "{}: path '{}' does not exist.",
                "Error".red(),
                &in_paths.display()
            );
            exit(0x0001);
        }

        // 2: check the path is actually a directory
        else if !in_paths.is_dir() {
            eprintln!(
                "{}: {} is not a directory.",
                "Error".red(),
                &in_paths.display()
            );
            exit(0x0001);
        }

        // 3: otherwise, give a warning about permissions
        else {
            eprintln!(
                "{}: permission denied for directory: {}",
                "Warning".yellow(),
                &in_paths.display()
            );
        }

    }

}

pub fn print_parallel(w: Walk) -> () {

    // set up worker & stealer
    let (mut worker, stealer): (chase_lev::Worker<DirEntry>, chase_lev::Stealer<DirEntry>) = chase_lev::deque();

    // set up parallel stealers
    let stealer2 = stealer.clone();
    let stealer3 = stealer.clone();
    let stealer4 = stealer.clone();
    let stealer5 = stealer.clone();

    let in_paths = w.path;

    let child_producer = thread::spawn(move || {
        Walk::push_subdir(in_paths, &mut worker);
    });

    // start popping off values
    // make an enum to signal when we're done instead :)
    // IDEA: have data return whatever was uneaten afterwards? Could handle nesting really well :)

    let child_consumer = thread::spawn(move || {
        while let chase_lev::Steal::Data(p) = stealer.steal() {
            println!("path: {}, size: {}", p.path().to_str().unwrap(), p.metadata().unwrap().len());}
    });

    let child2 = thread::spawn(move || {
        while let chase_lev::Steal::Data(p) = stealer2.steal() {
            println!("path: {}, size: {}", p.path().to_str().unwrap(), p.metadata().unwrap().len());}
    });

    let child3 = thread::spawn(move || {
        while let chase_lev::Steal::Data(p) = stealer3.steal() {
            println!("path: {}, size: {}", p.path().to_str().unwrap(), p.metadata().unwrap().len());}
    });

    let child4 = thread::spawn(move || {
        while let chase_lev::Steal::Data(p) = stealer4.steal() {
            println!("path: {}, size: {}", p.path().to_str().unwrap(), p.metadata().unwrap().len());}
    });

    let child5 = thread::spawn(move || {
        while let chase_lev::Steal::Data(p) = stealer5.steal() {
            println!("path: {}, size: {}", p.path().to_str().unwrap(), p.metadata().unwrap().len());}
    });

    let _ = child_producer.join();
    let _ = child_consumer.join();
    let _ = child2.join();
    let _ = child3.join();
    let _ = child4.join();
    let _ = child5.join();
}
