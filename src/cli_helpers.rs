extern crate regex;

use std::path::PathBuf;

pub fn get_depth(depth_from_cli: Option<&str>) -> u8 {
    if let Some(n) = depth_from_cli {
        n.parse::<u8>().expect("Please enter a positive whole number")
    }
    else {
        2
    }
}

pub fn get_num(num_from_cli: Option<&str>) -> usize {
    if let Some(num) = num_from_cli {
        num.parse::<usize>().expect("Please enter a positive whole number")
    }
    else {
        20
    }
}

pub fn get_dir(path_from_cli: Option<&str>) -> PathBuf {
    if let Some(read) = path_from_cli {
        PathBuf::from(read)
    }
    else {
        PathBuf::from(".")
    }
}
