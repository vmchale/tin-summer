#![allow(unused_imports)]
extern crate test;

use clap::App;
use std::fs;
use regex::Regex;
use std::path::PathBuf;
use std::mem::replace;
use test::test::Bencher;
use prelude::*;
use gitignore::*;
use std::fs::File;
use std::io::prelude::*;
use regex::RegexSet;
use colored::*;

#[bench]
fn bench_cli_options(b: &mut Bencher) {
    let yaml = load_yaml!("cli/options-en.yml");
    b.iter(|| {
        App::from_yaml(yaml)
            .version(crate_version!())
            .get_matches_from(vec!["sn", "ar", "."])
    })
}

#[bench]
fn bench_to_string(b: &mut Bencher) {
    let path = PathBuf::from("src/testdata/junk.rlib");
    b.iter(|| path.as_path().to_str().unwrap())
}

#[test]
fn cabal_regex_ignore() {
    let file_contents = include_str!("testdata/gitignore-tests/cabal-gitignore");
    let path = PathBuf::from("testdata/gitignore-tests/cabal-gitignore");
    let _ = file_contents_to_regex(file_contents, &path);
}

#[bench]
fn bench_gitignore(b: &mut Bencher) {
    let file_contents = include_str!("testdata/.gitignore");
    b.iter(|| {
        file_contents_to_regex(file_contents, &PathBuf::from("testdata/.gitignore"))
    })
}

#[bench]
fn bench_processors(b: &mut Bencher) {
    b.iter(|| get_processors())
}

#[bench]
fn bench_traversal_size(b: &mut Bencher) {
    let p = PathBuf::from(".");
    b.iter(|| read_size(&p, 0, None, &None, false))
}

#[bench]
fn bench_traversal_size_walkdir(b: &mut Bencher) {
    let p = ".";
    b.iter(|| read_all_walkdir(&p, None))
}
#[bench]
fn bench_traversal_all(b: &mut Bencher) {
    let p = PathBuf::from(".");
    b.iter(|| read_all(&p, 0, None, None, &None, false))
}

#[bench]
fn bench_traversal(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, &None, false))
}

#[bench]
fn bench_traversal_sort(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| {
        let v = read_all(&p, 4, None, None, &None, false);
        v.sort(None, None, false, None)
    })
}

#[bench]
fn bench_traversal_artifacts(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, &None, true))
}

#[bench]
fn bench_extension_regex(b: &mut Bencher) {
    let metadata = fs::metadata("src/main.rs").unwrap();
    b.iter(|| {
        is_artifact(
            "libdoggo.rlib",
            "target/release/libdoggo.rlib",
            &metadata,
            &None,
        )
    })
}

#[bench]
fn get_entries(b: &mut Bencher) {
    b.iter(|| fs::read_dir(".").unwrap())
}

#[bench]
fn count_entries(b: &mut Bencher) {
    b.iter(|| {
        let paths = fs::read_dir(".").unwrap();
        paths.count()
    })
}

#[bench]
fn get_entries_str(b: &mut Bencher) {
    b.iter(|| {
        let paths = fs::read_dir(".").unwrap();
        for p in paths {
            let _ = p.unwrap().path().to_str().unwrap();
        }
    })
}

#[bench]
fn get_entries_metadata(b: &mut Bencher) {
    b.iter(|| {
        let paths = fs::read_dir(".").unwrap();
        for p in paths {
            let _ = p.unwrap().metadata().unwrap();
        }
    })
}

#[bench]
fn get_entries_is_file(b: &mut Bencher) {
    b.iter(|| {
        let paths = fs::read_dir(".").unwrap();
        for p in paths {
            let val = p.unwrap();
            let t = val.file_type().unwrap();
            let _ = if t.is_file() { true } else { t.is_dir() };
        }
    })
}

#[bench]
fn bench_get_metadata(b: &mut Bencher) {
    b.iter(|| fs::metadata("src/main.rs").unwrap())
}

#[bench]
fn bench_extension_regex_long(b: &mut Bencher) {
    let metadata = fs::metadata("src/main.rs").unwrap();
    b.iter(|| {
        is_artifact(
            "sniff",
            "target/arm-unknown-linux-musleabi/release/sniff",
            &metadata,
            &None,
        )
    })
}

#[test]
fn test_parser() {
    let cli_input = "30M";
    assert_eq!(Some(30 * 1024 * 1024), threshold(Some(cli_input)));
}

#[bench]
fn bench_parser(b: &mut Bencher) {
    let cli_input = "1M";
    b.iter(|| threshold(Some(cli_input)))
}
