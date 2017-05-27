extern crate pad;

use pad::PadStr;
use std::fs;
use std::fmt;
use std::path::PathBuf;

// create a struct for displaying filenames/sizes in columns?
#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
struct FileSize {
    size: u64,
}

impl FileSize {
    pub fn new(i: u64) -> FileSize {
        FileSize { size: i }
    }
    pub fn add(mut self, other: FileSize) -> () {
        self.size += other.size;
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

fn read_files(in_paths: PathBuf) -> FileSize {
    let paths = fs::read_dir(in_paths).unwrap();
    let total_size = FileSize::new(0);

    for p in paths {
        // TODO instead write error message w/ implementation of the Display trait
        let path = p.unwrap().path();
        let metadata = fs::metadata(path.clone()).unwrap();

        // print out file size for a file
        if metadata.is_file() {
            let file_size = FileSize::new(metadata.len());
            total_size.add(file_size);
            let to_formatted = format!("{}", file_size);
            println!("{} {}", &to_formatted.pad_to_width(8), path.display());
        }
        // otherwise, go deeper
        // TODO consider using glob or regex! w/ accompanying functions.
        else if metadata.is_dir() {
            let dir_size = read_files(path.clone());
            total_size.add(dir_size); // also consider adding everything up nicely?
            let to_formatted = format!("{}", dir_size);
            println!("{} {}", &to_formatted.pad_to_width(8), path.display());
        }
    }
    total_size
}

fn main() {
    let init_dir = PathBuf::from("./");
    read_files(init_dir);
}
