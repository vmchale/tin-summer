#![feature(test)]
extern crate test;
#[macro_use]
extern crate clap;
extern crate liboskar;

use std::fs;
use std::path::PathBuf;
use test::test::Bencher;

use clap::App;

use liboskar::gitignore::*;
use liboskar::prelude::*;

#[bench]
fn bench_cli_options(b: &mut Bencher) {
    let yaml = load_yaml!("../src/cli/options-en.yml");
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

#[bench]
fn bench_gitignore(b: &mut Bencher) {
    let file_contents = include_str!("../src/testdata/.gitignore");
    b.iter(|| file_contents_to_regex(file_contents, &PathBuf::from("testdata/.gitignore")))
}

#[bench]
fn bench_processors(b: &mut Bencher) {
    b.iter(|| get_processors())
}

#[bench]
fn bench_traversal_size(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_size(&p, None, &None, false, false))
}

#[bench]
fn bench_traversal(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, &None, false, false, false))
}

#[bench]
fn bench_traversal_sort(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| {
        let v = read_all(&p, 4, None, None, &None, false, false, false);
        v.sort(None, None, false, None)
    })
}

#[bench]
fn bench_traversal_artifacts(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, &None, false, true, false))
}

#[bench]
fn bench_extension_regex(b: &mut Bencher) {
    let metadata = fs::metadata("src/main.rs").unwrap();
    b.iter(|| {
        is_artifact(
            "libdoggo.rlib",
            "target/release/libdoggo.rlib",
            &metadata,
            false,
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
fn bench_parser(b: &mut Bencher) {
    let cli_input = "1M";
    b.iter(|| threshold(Some(cli_input)))
}
