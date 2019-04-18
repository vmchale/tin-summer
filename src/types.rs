extern crate pad;

use self::pad::PadStr;
use colored::*;
use std::cmp::Ordering;
use std::fmt;
use std::path::PathBuf;

/// This is just a wrapper around a `u64` so that we can implement our own `Display` trait for our
/// file sizes.
#[derive(Ord, Eq, PartialOrd, PartialEq, Copy, Clone)]
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

    pub fn get(self) -> u64 {
        self.size
    }
}

#[derive(Debug)]
pub struct NamePair {
    pub bytes: FileSize,
    depth: u8,
    pub name: String,
    is_dir: bool,
}

fn sort_by_size(fst: &NamePair, snd: &NamePair) -> Ordering {
    fst.bytes.cmp(&snd.bytes)
}

impl NamePair {
    pub fn new(path: String, bytes_in: FileSize, d: u8, b: bool) -> NamePair {
        NamePair {
            name: path,
            bytes: bytes_in,
            depth: d,
            is_dir: b,
        }
    }
}

pub struct FileTree {
    pub file_size: FileSize,
    files: Vec<NamePair>,
}

pub fn display_item<T: fmt::Display>(name: &T, bytes: FileSize, display_bytes: bool) {
    if bytes != FileSize::new(0) {
        let to_formatted = if display_bytes {
            format!("{:?}", bytes)
        } else {
            format!("{}", bytes)
        };
        println!("{}\t {}", &to_formatted.green(), name);
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FileTree {
    pub fn sort(
        mut self,
        maybe_num: Option<usize>,
        min_bytes: Option<u64>,
        dirs_only: bool,
        max_depth: Option<u8>,
    ) -> FileTree {
        let self_size = if Some(self.file_size) > min_bytes.map(FileSize::new) {
            self.file_size
        } else {
            FileSize::new(0)
        };

        // filter by depth & truncate
        if let Some(n) = maybe_num {
            self.files.sort_by(|a, b| sort_by_size(b, a));
            let new = self
                .files
                .into_iter()
                .filter(|a| {
                    (if dirs_only { a.is_dir } else { true })
                        && Some(a.bytes) > min_bytes.map(FileSize::new)
                        && (max_depth.is_none() || Some(a.depth) <= max_depth)
                })
                .take(n)
                .collect::<Vec<NamePair>>();
            FileTree {
                file_size: self_size,
                files: new,
            }
        }
        // sort by size and filter by depth
        else {
            self.files.sort_by(|a, b| sort_by_size(a, b));
            let new = self
                .files
                .into_iter()
                .filter(|a| {
                    (if dirs_only { a.is_dir } else { true })
                        && Some(a.bytes) > min_bytes.map(FileSize::new)
                        && (max_depth.is_none() || Some(a.depth) <= max_depth)
                })
                .collect::<Vec<NamePair>>();
            FileTree {
                file_size: self_size,
                files: new,
            }
        }
    }

    pub fn filtered(
        mut self,
        min_bytes: Option<u64>,
        dirs_only: bool,
        max_depth: Option<u8>,
    ) -> FileTree {
        let self_size = if Some(self.file_size) > min_bytes.map(FileSize::new) {
            self.file_size
        } else {
            FileSize::new(0)
        };

        self.files = self
            .files
            .into_iter()
            .filter(|a| {
                (if dirs_only { a.is_dir } else { true })
                    && Some(a.bytes) > min_bytes.map(FileSize::new)
                    && (max_depth.is_none() || Some(a.depth) <= max_depth)
            })
            .collect::<Vec<NamePair>>();

        FileTree {
            file_size: self_size,
            files: self.files,
        }
    }

    pub fn new() -> FileTree {
        FileTree {
            file_size: FileSize::new(0),
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, size: FileSize) -> () {
        self.file_size.add(size);
    }

    pub fn push(
        &mut self,
        path: String,
        size: FileSize,
        subtree: Option<&mut FileTree>,
        depth: u8,
        is_dir: bool,
    ) -> () {
        // add to total
        self.file_size.add(size);

        // append subtree if appropriate
        if let Some(s) = subtree {
            self.files.append(&mut s.files);
        }

        // return new file tree
        self.files.push(NamePair::new(path, size, depth, is_dir));
    }

    pub fn display_tree(&mut self, init_dir: &PathBuf, with_bytes: bool) -> () {
        // display stuff
        let vec = &self.files;
        for name_pair in vec {
            display_item(&name_pair.name, name_pair.bytes, with_bytes);
        }

        display_item(&init_dir.display(), self.file_size, with_bytes);
    }
}

impl fmt::Debug for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pre_size = format!("{}", self.size);
        write!(f, "{} b", &pre_size.pad_to_width(8))
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.size < 1024 {
            let pre_size = format!("{}", self.size);
            write!(f, "{} b", &pre_size.pad_to_width(4))
        } else if self.size < 1048576 {
            let pre_size = if self.size / 1024 > 9 {
                format!("{}", self.size / 1024)
            } else if self.size as f32 / 1024.0 >= 9.95 {
                format!("{:.0}", self.size as f32 / 1024.0)
            } else {
                format!("{:.1}", self.size as f32 / 1024.0)
            };
            write!(f, "{} kB", &pre_size.pad_to_width(4))
        } else if self.size < 1073741824 {
            // 2^30
            let pre_size = if self.size / 1048576 > 9 {
                format!("{}", self.size / 1048576)
            } else if self.size as f32 / 1048576.0 >= 9.95 {
                format!("{:.0}", self.size as f32 / 1048576.0)
            } else {
                format!("{:.1}", self.size as f32 / 1048576.0)
            };
            write!(f, "{} MB", &pre_size.pad_to_width(4))
        } else if self.size < 1099511627776 {
            // 2^40
            let pre_size = if self.size / 1073741824 > 9 {
                format!("{}", self.size / 1073741824)
            } else if self.size as f32 / 1073741824.0 >= 9.95 {
                format!("{:.0}", self.size as f32 / 1073741824.0)
            } else {
                format!("{:.1}", self.size as f32 / 1073741824.0)
            };
            write!(f, "{} GB", &pre_size.pad_to_width(4))
        } else {
            // for 1 TB and above
            let pre_size = if self.size / 1099511627776 > 9 {
                format!("{}", self.size / 1099511627776)
            } else if self.size as f32 / 1099511627776.0 >= 9.95 {
                format!("{:.0}", self.size as f32 / 1099511627776.0)
            } else {
                format!("{:.1}", self.size as f32 / 1099511627776.0)
            };
            write!(f, "{} TB", &pre_size.pad_to_width(4))
        }
    }
}
