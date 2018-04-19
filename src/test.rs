use std::path::PathBuf;
use prelude::*;
use gitignore::*;

#[test]
fn cabal_regex_ignore() {
    let file_contents = include_str!("testdata/gitignore-tests/cabal-gitignore");
    let path = PathBuf::from("testdata/gitignore-tests/cabal-gitignore");
    let _ = file_contents_to_regex(file_contents, &path);
}

#[test]
fn bug_regex_ignore() {
    let file_contents = include_str!("testdata/gitignore-tests/another-gitignore");
    let path = PathBuf::from("testdata/gitignore-tests/another-gitignore");
    let reg = file_contents_to_regex(file_contents, &path);
    println!("{:?}", reg);
}

#[test]
fn test_parser() {
    let cli_input = "30M";
    assert_eq!(Some(30 * 1024 * 1024), threshold(Some(cli_input)));
}
