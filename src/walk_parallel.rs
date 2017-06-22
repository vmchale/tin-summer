#![allow(unused_imports)]

extern crate crossbeam;

use self::crossbeam::sync::MsQueue;
use regex::{RegexSet, Regex};
use types::{FileSize, NamePair};
use std::sync::Arc;
use std::io;
use std::thread;
use std::path::{PathBuf, Path};
use colored::*;
use ignore::{WalkBuilder, WalkState};
use std::sync::atomic::{AtomicU64, Ordering};
/*
struct Walk {
    path: PathBuf,
    gitignore: Option<RegexSet>,
    hgignore: Option<RegexSet>,
    darcs_boring: Option<RegexSet>,
    excludes: Option<Regex>,
    max_depth: Option<u8>,
    threshold: Option<u64>,
    // todo: add num_waiting and the like.
}

fn print_from_queue(queue_in: Arc<MsQueue<NamePair>>) -> () {

    let queue = queue_in.clone();
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::BufWriter::new(io::stdout());

        // ideally we should send a quit signal though??
        loop {
            let name_pair = queue.pop();
            if name_pair.bytes != FileSize::new(0) {
                let to_formatted = format!("{}", name_pair.bytes);
                println!("{}\t {}", &to_formatted.green(), name_pair.name);
            }
        }
    });

    stdout_thread.join().unwrap();
}

impl Walk {
    pub fn collect_all(self, threads: usize) -> Arc<MsQueue<NamePair>> {

        let files = Arc::new(MsQueue::new());
        let mut any_work = false;

        /*for w in workers {
            w.join().unwrap(); // if it fails to join to the main thread, that's bad.
        }*/

        files

    }

}
*/
pub fn read_parallel(
    in_paths: &Path,
    nproc: usize,
) -> FileSize {

    // create new walk + set file size to zero
    let mut builder = WalkBuilder::new(in_paths);
    let dir_size = Arc::new(AtomicU64::new(0));

    // set options for our walk
    builder.hidden(false)
        .follow_links(false)
        .ignore(false)
        .threads(nproc)
        .git_ignore(false)
        .git_exclude(false)
        .git_global(false)
        .parents(false);

    // runs loop
    builder.build_parallel().run(|| {

        let filesize_dir = dir_size.clone();
        Box::new(move |path| {

            if let Ok(p) = path {
                if p.path().is_file() {
                    if let Ok(metadata) = p.metadata() {
                        filesize_dir.fetch_add(metadata.len(), Ordering::Relaxed);
                    }
                    else {
                        eprintln!(
                            "{}: couldn't get metadata for {:?}",
                            "Warning".yellow(),
                            p.path()
                        )
                    }
                }
            } else if let Err(e) = path {
                eprintln!(
                    "{}: failed to get path data from:\n{:?}",
                    "Warning".yellow(),
                    e
                );
            };
            WalkState::Continue
        })

    });

    FileSize::new(Arc::try_unwrap(dir_size).unwrap().into_inner())

}

