extern crate pad;

use pad::PadStr;
use std::fs;
use std::fmt;
use std::path::PathBuf;

// create a struct for displaying filenames/sizes in columns?
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
}

impl NamePair {
    pub fn new(path: PathBuf, bytes_in: FileSize) -> NamePair {
        NamePair { name: path, bytes: bytes_in }
    }
    //fn cmp(&self, other: &NamePair) -> Ordering {
    //    self.bytes.cmp(&other.bytes)
    //}

}

pub struct FileTree {
    pub file_size: FileSize,
    files: Vec<NamePair>,
}

impl FileTree {
    pub fn sort(&mut self, maybe_num: Option<usize>) -> () {
        if let Some(n) = maybe_num {
            self.files.sort();
            self.files.reverse();
            self.files = self.files.clone().into_iter().take(n).collect();
        }
        else {
            self.files.sort();
            self.files.reverse();
        }

    }
    pub fn new() -> FileTree {
        FileTree { file_size: FileSize::new(0), files: Vec::new() }
    }
    pub fn push(&mut self, path: PathBuf, size: FileSize, subtree: Option<&mut FileTree>) -> () {
        self.file_size.add(size); // add subdirectory or file size to total
        if let Some(s) = subtree {
            self.files.append(&mut s.files); // add subtree if desired
        }
        self.files.push(NamePair::new(path, size)); // tag file or subdirectory with its size
        // by tracking total size nicely, we avoid the need to traverse the vector to sum it.
    }
    pub fn display_tree(&mut self, init_dir: PathBuf) -> () {
        // subdirs &c.
        let vec = &self.files;
        for ref name_pair in vec {
            let to_formatted = format!("{}", name_pair.bytes);
            println!("{} {}", &to_formatted.pad_to_width(8), name_pair.name.display());
        }

        // total
        let to_formatted = format!("{}", self.file_size);
        let path = init_dir.display(); // fix this!! better data structure
        println!("{} {}", &to_formatted.pad_to_width(8), path);
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

pub fn read_files(in_paths: PathBuf) -> FileTree {
    let paths = fs::read_dir(in_paths.clone()).unwrap();
    let mut tree = FileTree::new();
    let mut total_size = FileSize::new(0);

    for p in paths {
        let path = p.unwrap().path(); // TODO no unwraps
        let metadata = fs::metadata(path.clone()).unwrap();

        // append file size/name for a file
        if metadata.is_file() {
            let file_size = FileSize::new(metadata.len());
            tree.push(path.clone(), file_size, None);
            total_size.add(file_size);
        }
        // otherwise, go deeper
        else if metadata.is_dir() {
            let mut subtree = read_files(path.clone());
            let dir_size = subtree.file_size;
            tree.push(path, dir_size, Some(&mut subtree));
            total_size.add(dir_size);
        }
    }
    tree
}

