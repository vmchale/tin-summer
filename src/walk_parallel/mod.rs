#![allow(dead_code)]

extern crate crossbeam;

pub mod single_threaded;

use std::fs;
use self::crossbeam::sync::chase_lev;
use regex::{RegexSet, Regex};
use std::path::PathBuf;
use colored::*;
use std::process::exit;
use std::thread;

pub use walk_parallel::single_threaded::*;

pub enum Status<T> {
    Done,
    Data(T),
}

pub struct Walk {
    path: PathBuf,
    gitignore: Option<RegexSet>,
    hgignore: Option<RegexSet>,
    darcs_boring: Option<RegexSet>,
    excludes: Option<Regex>,
    max_depth: Option<usize>,
    threshold: Option<u64>,
    start_depth: usize,
    nproc: usize,
}

impl Walk {

    // force this to return a numerical value??
    pub fn print_dir(w: Walk) -> () {
        let v = read_all(
            &w.path,
            (w.start_depth as u8),
            Some(2),
            None,
            None,
            &None,
            false,
            false,
        );

        // filter by depth
        let mut v_filtered = v.filtered(None, true);

        v_filtered.display_tree(w.path);
    }

    fn get_proc(&self) -> usize {
        self.nproc
    }

    pub fn new(p: PathBuf, n: usize) -> Walk {
        Walk {
            path: p,
            gitignore: None,
            hgignore: None,
            darcs_boring: None,
            excludes: None,
            max_depth: None,
            threshold: None,
            start_depth: 0,
            nproc: n,
        }
    }

    fn bump_depth(&mut self) -> () {
        self.start_depth += 1;
    }

    pub fn push_subdir(w: &Walk, ref mut worker: &mut chase_lev::Worker<Status<Walk>>) {

        let in_paths = &w.path;

        // fill up queue + print out files
        if let Ok(paths) = fs::read_dir(in_paths) {

            //let dirs_only = paths.filter(|p|

            // iterate over all the entries in the directory
            for p in paths {
                let val = match p {
                    Ok(x) => x,
                    _ => {
                        eprintln!("{}: path error at {:?}.", "Error".red(), p);
                        exit(0x0001)
                    }
                };
                match val.file_type() {
                    Ok(t) => {
                        // possibility: if the number of directories is less than the number of
                        // cores, descend two levels!
                        if t.is_dir() {
                            let mut new_path = w.path.to_owned();
                            new_path.push(val.file_name());
                            let mut new_walk = Walk::new(new_path, w.get_proc());
                            new_walk.bump_depth();
                            worker.push(Status::Data(new_walk));
                        }
                    }
                    _ => {
                        eprintln!(
                            "{}: could not determine file type for: {}",
                            "Warning".yellow(),
                            val.file_name().to_str().unwrap()
                        )
                    }
                }
            }

            // send "done" messages to all the workers
            let iter = 0..(w.get_proc() - 1);
            iter.map(|_| worker.push(Status::Done)).count();

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
    let (mut worker, stealer): (
        chase_lev::Worker<Status<Walk>>,
        chase_lev::Stealer<Status<Walk>>,
    ) = chase_lev::deque();

    // set up our iterator for the workers
    let iter = 0..(&w.get_proc() - 1);

    // TODO the worker should pop values too
    let child_producer = thread::spawn(move || { 
        Walk::push_subdir(&w, &mut worker);
    });

    //
    let mut threads = Vec::new();

    // set up as many workers as we have threads
    let _ = iter.map(|_| {
        let stealer_clone = stealer.clone();
        let child_consumer = thread::spawn(move || loop {
            if let chase_lev::Steal::Data(p) = stealer_clone.steal() {
                match p {
                    Status::Data(d) => Walk::print_dir(d),
                    _ => break,
                };
            }
        });
        threads.push(child_consumer);
    }).count();

    // join the child producer to the main thread
    let _ = child_producer.join();

    // join the workers to the main thread
    for v in threads {
        v.join().unwrap();
    }

}
