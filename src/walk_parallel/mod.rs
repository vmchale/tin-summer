extern crate crossbeam;
extern crate walkdir;

pub mod single_threaded;

use self::crossbeam::deque::fifo;
use self::crossbeam::deque::Pop;
use self::crossbeam::deque::Steal;
use self::crossbeam::deque::Worker;
use self::walkdir::WalkDir;
use colored::*;
use error::*;
use regex::{Regex, RegexSet};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use types::FileSize;
use utils::size;

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
    get_blocks: bool,
    follow_symlinks: bool,
    artifacts_only: bool,
}

impl Walk {
    /// function to make output from a 'Walk', using one thread. It also takes an 'Arc<AtomicU64>'
    /// and will add the relevant directory sizes to it.
    pub fn print_dir(w: &Walk, total: &Arc<AtomicUsize>) {
        let excludes = match w.excludes {
            Some(ref x) => Some(x),
            _ => None,
        };

        let v = if excludes.is_some() || w.artifacts_only {
            read_all(
                &w.path,
                w.start_depth as u8,
                w.max_depth,
                excludes,
                &w.gitignore,
                false,
                w.artifacts_only,
            )
        } else {
            read_all_fast(&w.path, w.start_depth as u8, w.max_depth)
        };

        let subdir_size = v.file_size.get();

        total.fetch_add(subdir_size as usize, Ordering::Relaxed);

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

            v_filtered.display_tree(&w.path);
        }
    }

    /// set the maximum depth to display
    pub fn set_depth(&mut self, d: u8) {
        self.max_depth = Some(d);
    }

    /// set the regex for excludes
    pub fn set_regex(&mut self, r: Regex) {
        self.excludes = Some(r);
    }

    /// set the minumum file size
    pub fn set_threshold(&mut self, n: u64) {
        self.threshold = Some(n);
    }

    /// include files when printing
    pub fn with_files(&mut self) {
        self.show_files = true;
    }

    /// include files when printing
    pub fn blocks(&mut self) {
        self.get_blocks = true;
    }

    /// include files when printing
    pub fn artifacts_only(&mut self) {
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
            get_blocks: false,
            follow_symlinks: false,
            artifacts_only: false,
        }
    }

    fn bump_depth(&mut self) {
        self.start_depth += 1;
    }

    /// This takes a 'Walk' and a 'Worker<Status<Walk>>' and executes the walk *in parallel*,
    /// creating new work for each subdirectory. It's not the most efficient concurrency
    /// imaginable, but it's fast and easy-ish to use. It *also* takes in an 'Arc<AtomicU64>',
    /// which it updates with any file sizes in the directory.
    pub fn push_subdir(w: &Walk, worker: &mut Worker<Status<Walk>>, total: &Arc<AtomicUsize>) {
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
                                if w.excludes.is_some() {
                                    new_walk.set_regex(w.excludes.clone().unwrap());
                                }
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
                                    let size = size(&l, w.get_blocks);
                                    total.fetch_add(size as usize, Ordering::Relaxed);
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
                        _ => eprintln!(
                            "{}: could not determine file type for: {}",
                            "Warning".yellow(),
                            val.path().display()
                        ),
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
                "{}: path '{}' does not exist, or you do not have permission to enter.",
                "Error".red(),
                &in_paths.display()
            );
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
                let size = size(&l, w.get_blocks); // l.len();
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

fn ats_cgen(p: Option<&OsStr>) -> bool {
    lazy_static! {
        static ref DATS_C: Regex =
            Regex::new(r"(_(d|h|s)ats\.c|_lats\.dats|_sats\.c|_stub\.h)$").unwrap();
    }
    match p {
        Some(p) => DATS_C.is_match(&p.to_string_lossy().to_string()),
        None => false,
    }
}

fn latex_log<P: AsRef<Path>>(p: P) -> bool {
    lazy_static! {
        static ref LOG: Regex = Regex::new(r"\.log$").unwrap();
    }

    if LOG.is_match(&p.as_ref().to_string_lossy().to_string()) {
        let mut parent = (&p.as_ref())
            .parent()
            .unwrap()
            .to_string_lossy()
            .to_string();
        parent.push_str("/*.tex");
        glob_exists(&parent)
    } else {
        false
    }
}

// TODO figure out why the unwrap_or is failing?
// FIXME take optional reference to a regex
pub fn clean_project_dirs<P: AsRef<Path>>(p: P, exclude: &Option<Regex>, _: bool) {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"\.(a|i|ii|la|lo|o|keter|bc|dyn_o|d|rlib|crate|hi|hc|chi|dyn_hi|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|spl|bbl|blg|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|hspec-failures|pyc|mod|vo|beam|agdai|go\.(v|teak|xmldef|rewrittenast|rewrittengo|simplego|tree-(bind|eval|finish|parse))|p_hi|p_o|prof|hide-cache|ghc\.environment\..*\d.\d.\d|(t|p|m)ix|synctex\.gz|hl|sandbox\.config|hp|eventlog|ipa|ttc|chs\.h|chi|\d+\.actual|\d+\.expected)$")
            .unwrap();
    }
    lazy_static! {
        static ref SRC_CONTROL: Regex = Regex::new(r"(_darcs|\.(git|hg|pijul))").unwrap();
    }

    for dir in WalkDir::new(p)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|p| {
            exclude
                .clone()
                .map(|e| e.is_match(&p.path().to_string_lossy().to_string()))
                != Some(false)
        })
        .filter(|p| !SRC_CONTROL.is_match(&p.path().to_string_lossy().to_string()))
        .filter(|p| {
            REGEX.is_match(&p.path().to_string_lossy().to_string())
                || is_project_dir(
                    &p.path().to_string_lossy().to_string(),
                    &p.path()
                        .file_name()
                        .map(|x| x.to_string_lossy().to_string())
                        .unwrap_or_else(|| "".to_string()),
                )
                || latex_log(&p.path())
                || ats_cgen(p.path().file_name())
                || ({
                    let x = &p.path().to_string_lossy().to_string();
                    x.ends_with("/flxg_stats.txt")
                })
        })
    {
        if dir.file_type().is_file() {
            fs::remove_file(dir.path()).unwrap_or(());
        } else if dir.file_type().is_dir() {
            fs::remove_dir_all(dir.path()).unwrap_or(());
        }
    }
}

/// Given a 'Walk' struct, traverse it concurrently and print out any relevant outputs.
/// Currently, this only works for a depth of two, which is probably bad.
pub fn print_parallel(w: Walk) {
    // initialize the total at 0 and create a reference to it
    let val = AtomicUsize::new(0);
    let arc = Arc::new(val);
    let arc_producer = arc.clone();
    let arc_child = arc.clone();
    let path_display = w.path.clone();

    // set up worker & stealer
    let (mut worker, stealer) = fifo();

    // set up our iterator for the workers
    let iter = 0..(&w.get_proc() - 1);

    let _ = (&w.path).is_file();

    // create the producer in another thread
    let child_producer = thread::spawn(move || {
        let arc_local = arc_producer.clone();

        // assign work to everyone
        Walk::push_subdir(&w, &mut worker, &arc_local);

        // start popping off values in the worker's thread
        loop {
            if let Pop::Data(p) = worker.pop() {
                match p {
                    Status::Data(d) => Walk::print_dir(&d, &arc_local),
                    _ => break,
                }
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
            if let Steal::Data(p) = stealer_clone.steal() {
                match p {
                    Status::Data(d) => Walk::print_dir(&d, &arc_local),
                    _ => break,
                }
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
    let size = FileSize::new(m as u64);

    // print directory total.
    if size != FileSize::new(0) {
        let to_formatted = format!("{}", size);
        println!("{}\t {}", &to_formatted.green(), path_display.display());
    }
}
