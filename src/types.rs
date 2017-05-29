extern crate pad;
extern crate regex;

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

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct NamePair {
    bytes: FileSize,
    name: PathBuf,
    pub depth: u8,
}

impl NamePair {
    pub fn new(path: PathBuf, bytes_in: FileSize, d: u8) -> NamePair {
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

    pub fn sort(mut self, maybe_num: Option<usize>, d: u8) -> FileTree {
        if let Some(n) = maybe_num {
            self.files.sort();
            self.files.reverse();
            let new = self.files.into_iter()
                .filter(|a| a.depth <= d )
                .take(n).collect::<Vec<NamePair>>();
            FileTree { file_size: self.file_size, files: new }
        }
        else {
            self.files.sort();
            self.files.reverse();
            FileTree { file_size: self.file_size, files: self.files }
        }
    }

    pub fn filtered(mut self, d: u8) -> FileTree {
        self.files = self.files.into_iter() // TODO intelligent sorting w/ filters based on 
            .filter(|a| a.depth <= d)
            .collect::<Vec<NamePair>>();
        FileTree { file_size: self.file_size, files: self.files }
    }

    pub fn new() -> FileTree {
        FileTree { file_size: FileSize::new(0), files: Vec::new() }
    }
    
    pub fn push(&mut self, path: PathBuf, size: FileSize, subtree: Option<&mut FileTree>, depth: u8) -> () {
        self.file_size.add(size); // add subdirectory or file size to total
        if let Some(s) = subtree {
            self.files.append(&mut s.files); // add subtree if desired
        }
        self.files.push(NamePair::new(path, size, depth));
    }

    pub fn display_tree(&mut self, init_dir: PathBuf) -> () {
        // subdirs &c.
        let vec = &self.files;
        for name_pair in vec {
            if name_pair.bytes != FileSize::new(0) {
                let to_formatted = format!("{}", name_pair.bytes);
                println!("{}\t {}", &to_formatted.green(), name_pair.name.display());
            }
        }

        // total; don't display if it's zero because we used threshholds
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
