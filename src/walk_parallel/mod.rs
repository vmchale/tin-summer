#![allow(dead_code)]

extern crate crossbeam;

pub mod single_threaded;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::fs;
use self::crossbeam::sync::chase_lev;
use regex::{RegexSet, Regex};
use std::path::PathBuf;
use colored::*;
use std::process::exit;
use std::thread;
use error::*;
use types::FileSize;

pub use walk_parallel::single_threaded::*;

/// Enum for messaging between workers/stealers
pub enum Status<T> {
    Done,
    Data(T),
}

/// The 'Walk' struct contains all the information we need to traverse a directory.
#[derive(Debug)]
pub struct Walk {
    pub path: PathBuf,
    gitignore: Option<RegexSet>,
    hgignore: Option<RegexSet>,
    darcs_boring: Option<RegexSet>,
    excludes: Option<Regex>,
    max_depth: Option<u8>,
    threshold: Option<u64>,
    start_depth: usize,
    nproc: usize,
    show_files: bool,
    follow_symlinks: bool,
    artifacts_only: bool,
}

impl Walk {
    /// function to make output from a 'Walk', using one thread. It also takes an 'Arc<AtomicU64>'
    /// and will add the relevant directory sizes to it.
    pub fn print_dir(w: Walk, total: Arc<AtomicU64>) -> () {

        let excludes = match w.excludes {
            Some(ref x) => Some(x),
            _ => None,
        };

        let v = read_all(
            &w.path,
            (w.start_depth as u8),
            w.max_depth,
            excludes,
            &w.gitignore,
            w.artifacts_only,
        );

        let subdir_size = v.file_size.get();

        total.fetch_add(subdir_size, Ordering::Relaxed);

        let mut to_print = if let Some(m) = w.threshold {
            subdir_size > m
        } else {
            true
        };

        if let Some(0) = w.max_depth {
            to_print = false;
        }

        if to_print {
            // filter by depth
            let mut v_filtered = v.filtered(w.threshold, !w.show_files, w.max_depth);

            v_filtered.display_tree(w.path);
        }
    }

    /// set the maximum depth to display
    pub fn set_depth(&mut self, d: u8) -> () {
        self.max_depth = Some(d);
    }

    /// set the regex for excludes
    pub fn set_regex(&mut self, r: Regex) -> () {
        self.excludes = Some(r);
    }

    /// set the minumum file size
    pub fn set_threshold(&mut self, n: u64) -> () {
        self.threshold = Some(n);
    }

    /// include files when printing
    pub fn with_files(&mut self) -> () {
        self.show_files = true;
    }

    /// include files when printing
    pub fn artifacts_only(&mut self) -> () {
        self.artifacts_only = true;
    }

    fn get_proc(&self) -> usize {
        self.nproc
    }

    /// Create a new 'Walk' from a 'PathBuf' and the number
    /// of processor cores to be used.
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
            show_files: false,
            follow_symlinks: false,
            artifacts_only: false,
        }
    }

    fn bump_depth(&mut self) -> () {
        self.start_depth += 1;
    }

    /// This takes a 'Walk' and a 'Worker<Status<Walk>>' and executes the walk *in parallel*,
    /// creating new work for each subdirectory. It's not the most efficient concurrency
    /// imaginable, but it's fast and easy-ish to use. It *also* takes in an 'Arc<AtomicU64>',
    /// which it updates with any file sizes in the directory.
    pub fn push_subdir(
        w: &Walk,
        mut worker: &mut chase_lev::Worker<Status<Walk>>,
        total: Arc<AtomicU64>,
    ) {

        let in_paths = &w.path;

        // fill up queue + print out files
        if let Ok(paths) = fs::read_dir(in_paths) {

            // iterate over all the entries in the directory
            for p in paths {
                let val = match p {
                    Ok(x) => x,
                    _ => {
                        eprintln!("{}: path error at {:?}.", "Error".red(), p);
                        exit(0x0001)
                    }
                };

                let exclude_check = if let Some(ref x) = w.excludes {
                    if let Some(r) = val.path().into_os_string().to_str() {
                        !x.is_match(r)
                    } else {
                        eprintln!(
                            "{}: ignoring invalid unicode at: {:?}",
                            "Warning".yellow(),
                            val.path().display()
                        );
                        true
                    }
                } else {
                    true
                };

                if exclude_check {
                    match val.file_type() {
                        Ok(t) => {
                            if t.is_dir() {
                                let mut new_path = w.path.to_owned();
                                new_path.push(val.file_name());
                                let mut new_walk = Walk::new(new_path, w.get_proc());
                                if w.show_files {
                                    new_walk.with_files();
                                }
                                new_walk.bump_depth();
                                if let Some(d) = w.max_depth {
                                    new_walk.set_depth(d);
                                }
                                if let Some(b) = w.threshold {
                                    new_walk.set_threshold(b);
                                }
                                worker.push(Status::Data(new_walk)); // pass a vector of Arc's to do 2-level traversals?
                            } else if t.is_file() {
                                if let Ok(l) = val.metadata() {
                                    let size = l.len();
                                    total.fetch_add(size, Ordering::Relaxed);
                                    if w.show_files && size != 0 {
                                        let to_formatted = format!("{}", FileSize::new(size));
                                        println!(
                                            "{}\t {}",
                                            &to_formatted.green(),
                                            val.path().display()
                                        );
                                    }
                                } else {
                                    eprintln!(
                                        "{}: could not find filesize for file at {}.",
                                        "Warning".yellow(),
                                        val.path().display()
                                    );
                                }
                            }
                        }
                        _ => {
                            eprintln!(
                                "{}: could not determine file type for: {}",
                                "Warning".yellow(),
                                val.path().display()
                            )
                        }
                    }
                }
            }

            // send "done" messages to all the workers
            let iter = 0..(w.get_proc());
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
            if w.artifacts_only {
                eprintln!(
                    "{}: {} is not a directory; not searching for artifacts",
                    "Warning".yellow(),
                    &in_paths.display()
                );
            }

            if let Ok(l) = in_paths.metadata() {
                let size = l.len();
                let to_formatted = format!("{}", FileSize::new(size));
                println!("{}\t {}", &to_formatted.green(), in_paths.display());
            } else {
                panic!("{}", Internal::IoError);
            }
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

/// Given a 'Walk' struct, traverse it concurrently and print out any relevant outputs.
/// Currently, this only works for a depth of two, which is probably bad.
pub fn print_parallel(w: Walk) -> () {

    // initialize the total at 0 and create a reference to it
    let val = AtomicU64::new(0);
    let arc = Arc::new(val);
    let arc_producer = arc.clone();
    let arc_child = arc.clone();
    let path_display = w.path.clone();

    // set up worker & stealer
    let (mut worker, stealer): (
        chase_lev::Worker<Status<Walk>>,
        chase_lev::Stealer<Status<Walk>>,
    ) = chase_lev::deque();

    // set up our iterator for the workers
    let iter = 0..(&w.get_proc() - 1);

    // create the producer in another thread
    let child_producer = thread::spawn(move || {

        let arc_local = arc_producer.clone();

        // assign work to everyone
        Walk::push_subdir(&w, &mut worker, arc_local.clone());

        // start popping off values in the worker's thread
        loop {
            if let Some(p) = worker.try_pop() {
                match p {
                    Status::Data(d) => Walk::print_dir(d, arc_local.clone()),
                    _ => break,
                };
            }
        }

    });

    // create a vector of thread handles so that it doesn't execute
    // everything sequentially.
    let mut threads = Vec::new();

    // set up as many workers as we have threads
    for _ in iter {

        // create a new stealer
        let stealer_clone = stealer.clone();

        let arc_local = arc_child.clone();

        // run the stealer in a new thread
        let child_consumer = thread::spawn(move || loop {

            if let chase_lev::Steal::Data(p) = stealer_clone.steal() {
                match p {
                    Status::Data(d) => Walk::print_dir(d, arc_local.clone()),
                    _ => break,
                };
            }
        });

        threads.push(child_consumer);

    }

    // join the child producer to the main thread
    let _ = child_producer.join();

    // join the workers to the main thread
    let _ = threads
        .into_iter()
        .map(|v| {
            let result = v.join();
            if let Ok(exit) = result {
                exit
            } else if let Err(e) = result {
                panic!("{:?}", e)
            }
        })
        .count();

    // get the total size
    let m = arc.load(Ordering::SeqCst); // TODO - check if this works with Relaxed?
    let size = FileSize::new(m);

    // print directory total.
    if size != FileSize::new(0) {
        let to_formatted = format!("{}", size);
        println!("{}\t {}", &to_formatted.green(), path_display.display());
    }

}
