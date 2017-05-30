#![allow(unused_imports)]
extern crate test;
extern crate regex;

use regex::Regex;
use std::path::PathBuf;
use std::mem::replace;
use test::test::Bencher;
use prelude::*;

#[bench]
fn bench_traversal(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, None, true, false))
}

#[bench]
fn bench_traversal_artifacts(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, None, true, true))
}

#[bench]
fn bench_extension_regex(b: &mut Bencher) {
    let p = PathBuf::from("target/release/libdoggo.rlib");
    b.iter(|| is_artifact(&p, None) )
}

#[bench]
fn bench_extension_regex_long(b: &mut Bencher) {
    let p = PathBuf::from("./target/release/.fingerprint/kernel32-sys-5ee1259db1228dbc/build-script-build_script_build-5ee1259db1228dbc.json");
    b.iter(|| is_artifact(&p, None) )
}

#[bench]
fn bench_parser(b:&mut Bencher) {
    let cli_input = "1M";
    b.iter(|| threshhold(Some(cli_input)))
}
