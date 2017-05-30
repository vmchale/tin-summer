extern crate pad;
extern crate regex;

use std::cmp::Ordering;
use self::pad::PadStr;
use std::fmt;
use std::path::PathBuf;
use colored::*;

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct FileSize {
    size: u64,
}

impl FileSize {

    pub fn new(i: u64) -> FileSize {
        FileSize { size: i }
    }

    pub fn add(&mut self, other: FileSize) -> () {
        self.size += other.size;
    }

}

pub struct NamePair {
    bytes: FileSize,
    depth: u8,
    name: String,
}

fn sort_by_size(fst: &NamePair, snd: &NamePair) -> Ordering {
    fst.bytes.cmp(&snd.bytes)
}

impl NamePair {
    pub fn new(path: String, bytes_in: FileSize, d: u8) -> NamePair {
        NamePair { name: path, bytes: bytes_in, depth: d }
    }
}

pub struct FileTree {
    pub file_size: FileSize,
    files: Vec<NamePair>,
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FileTree {

    // for whatever reason, it's faster to display smallest files first
    pub fn sort(mut self, maybe_num: Option<usize>, d: u8) -> FileTree {

        // filter by depth & truncate
        if let Some(n) = maybe_num {
            self.files.sort_by(|a, b| sort_by_size(b, a));
            let new = self.files.into_iter()
                .filter(|a| a.depth <= d )
                .take(n).collect::<Vec<NamePair>>();
            FileTree { file_size: self.file_size, files: new }
        }

        // sort by size and filter by depth
        else {
            self.files.sort_by(|a, b| sort_by_size(a, b));
            let new = self.files.into_iter()
                .filter(|a| a.depth <= d )
                .collect::<Vec<NamePair>>();
            FileTree { file_size: self.file_size, files: new }
        }
    }

    pub fn filtered(mut self, d: u8) -> FileTree {
        self.files = self.files.into_iter()
            .filter(|a| a.depth <= d)
            .collect::<Vec<NamePair>>();
        FileTree { file_size: self.file_size, files: self.files }
    }

    pub fn new() -> FileTree {
        FileTree { file_size: FileSize::new(0), files: Vec::new() }
    }
    
    pub fn push(&mut self, path: String, size: FileSize, subtree: Option<&mut FileTree>, depth: u8) -> () {

        // add to total
        self.file_size.add(size);

        // append subtree
        if let Some(s) = subtree {
            self.files.append(&mut s.files); // add subtree if desired
        }

        // return new file tree
        self.files.push(NamePair::new(path, size, depth));

    }

    pub fn display_tree(&mut self, init_dir: PathBuf) -> () {

        // display stuff
        let vec = &self.files;
        for name_pair in vec {
            if name_pair.bytes != FileSize::new(0) {
                let to_formatted = format!("{}", name_pair.bytes);
                println!("{}\t {}", &to_formatted.green(), name_pair.name);
            }
        }

        // display total if it's nonzero
        if self.file_size != FileSize::new(0) {
            let to_formatted = format!("{}", self.file_size);
            let path = init_dir.display();
            println!("{}\t {}", &to_formatted.green(), path);
        }

    }

}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.size < 1024 {
            let pre_size = format!("{}", self.size);
            write!(f, "{} b", &pre_size.pad_to_width(4))
        }

        else if self.size < 1048576 { // 2^20 = 1024 * 1024
            let pre_size = format!("{}", self.size/1025);
            write!(f, "{} kB", &pre_size.pad_to_width(4))
        }

        else if self.size < 1073741824 { // 2^30
            let pre_size = format!("{}", self.size/1048576);
            write!(f, "{} MB", &pre_size.pad_to_width(4))
        }

        else if self.size < 1099511627776 { // 2^40
            let pre_size = format!("{}", self.size/1073741824);
            write!(f, "{} GB", &pre_size.pad_to_width(4))
        }

        else { // for 1 TB and above
            let pre_size = format!("{}", self.size/1099511627776);
            write!(f, "{} TB", &pre_size.pad_to_width(4))
        }
    }
}
