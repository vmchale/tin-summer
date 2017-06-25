use std::fs;
use regex::{RegexSet, Regex};
use utils::*;
use std::path::PathBuf;
use colored::*;
use std::process::exit;
use types::*;
use std::fs::Metadata;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;

/// Helper function to determine whether a path points
///
/// Rules:
/// - if the file extension of that is that of an artifact, return true
/// - if the file is executable and included in the .gitignore, return true
/// - return false otherwise
///
/// Explanation of extensions:
/// - `.a`, `.la`, `.o`, `.lo`, `.so.*`:
/// - `.S`: assembly
/// - .ll, `.bc`: llvm
/// - `.keter`: keter
/// - `.d`: make
/// - `.rlib`, `.crate`: rust
/// - `.hi`, `.dyn_hi`, `.dyn_o`, `.p_hi`, `.p_o`, `.prof`, `.dump-.*`, `.tix`: GHC
/// - `.webapp`: Web app manifest
/// - `.js.externs`, `.jsexe`, `.min.js`:
/// - `.ibc`: Idris
/// - `.toc`, `.aux`, `.fdb_latexmk`, `.fls`: TeX
/// - `.egg-info`, `.whl`, `.pyc`: python
/// - `.js_a`, `.js_hi`, `.js_o`: GHCJS
/// - `.vmb`: Vim
/// - `.crx`: chrome
/// - `.elmo`, `.elmi`: Elm
/// - `.mod`: FORTRAN
/// - `.ji`, `.jld`: julia
#[cfg(not(target_os = "windows"))]
pub fn is_artifact(
    path_str: &str,
    full_path: &str,
    re: Option<&Regex>,
    metadata: &Metadata,
    gitignore: &Option<RegexSet>,
) -> bool {

    lazy_static! {
        static ref REGEX_GITIGNORE: Regex = 
            Regex::new(r".*?\.(stats|conf|h|out|cache.*|dat|pc|info|\.js)$")
            .unwrap();
    }

    // match on the user's expression if it exists
    if let Some(r) = re {
        if r.is_match(path_str) {
            true
        } else if let &Some(ref x) = gitignore {
            if metadata.permissions().mode() == 0o755 || REGEX_GITIGNORE.is_match(path_str) {
                x.is_match(full_path)
            } else {
                false
            }
        } else {
            false
        }
    }
    // otherwise, use builtin expressions
    else {

        lazy_static! {
            static ref REGEX: Regex = 
                Regex::new(r".*?\.(a|la|lo|o|ll|keter|bc|dyn_o|d|rlib|crate|min\.js|hi|dyn_hi|S|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$")
                .unwrap();
        }

        if REGEX.is_match(path_str) {
            true
        } else if let &Some(ref x) = gitignore {
            if metadata.permissions().mode() == 0o755 || REGEX_GITIGNORE.is_match(path_str) {
                x.is_match(full_path)
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_artifact(
    path_str: &str,
    full_path: &str,
    re: Option<&Regex>,
    _: &Metadata,
    gitignore: &Option<RegexSet>,
) -> bool {

    lazy_static! {
        static ref REGEX_GITIGNORE: Regex = 
            Regex::new(r".*?\.(stats|conf|h|out|cache.*|dat|pc|info|\.js)$")
            .unwrap();
    }

    if let Some(r) = re {
        if r.is_match(path_str) {
            true
        } else if let &Some(ref x) = gitignore {
            if REGEX_GITIGNORE.is_match(path_str) {
                x.is_match(full_path)
            } else {
                false
            }
        } else {
            false
        }
    } else {

        lazy_static! {
            static ref REGEX: Regex = 
                Regex::new(r".*?\.(exe|a|la|o|ll|keter|bc|dyn_o|d|rlib|crate|min\.js|hi|dyn_hi|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$")
                .unwrap();
        }

        if REGEX.is_match(&path_str) {
            true
        } else if let &Some(ref x) = gitignore {
            if REGEX_GITIGNORE.is_match(path_str) {
                x.is_match(full_path)
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Function to process directory contents and return a `FileTree` struct.
pub fn read_size(
    in_paths: &PathBuf,
    depth: u8,
    artifact_regex: Option<&Regex>,
    excludes: Option<&Regex>,
    maybe_gitignore: &Option<RegexSet>,
    with_gitignore: bool,
    artifacts_only: bool,
) -> FileSize {

    // attempt to read the .gitignore
    let mut size = FileSize::new(0);
    let gitignore = if with_gitignore {
        mk_ignores(in_paths, maybe_gitignore)
    } else {
        None
    };

    // for project directories
    lazy_static! {
        static ref REGEX_PROJECT_DIR: Regex = 
            Regex::new(r"(.stack-work|dist|dist-newstyle|target|.*\.egg-info|elm-stuff)$")
            .unwrap();
    }

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            let val = match p {
                Ok(x) => x,
                _ => {
                    eprintln!("{}: path error at {:?}.", "Error".red(), p);
                    exit(0x0001)
                }
            };
            let path = val.path();
            let (path_string, bool_loop): (&str, bool) = if let Some(x) = path.as_path().to_str() {
                let bool_loop = match excludes {
                    Some(ex) => !ex.is_match(x),
                    _ => true,
                };
                (x, bool_loop)
            } else {
                eprintln!(
                    "{}: skipping invalid unicode filepath at {:?}",
                    "Warning".yellow(),
                    path
                );
                ("", false)
            };

            // only consider path if we're not using regex excludes or
            // if they don't match the exclusion regex
            if bool_loop {

                let path_type = val.file_type().unwrap();

                // append file size/name for a file
                if path_type.is_file() {
                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = val.metadata() {
                        // faster on Windows
                        if !artifacts_only ||
                            {
                                is_artifact(
                                    val.file_name().to_str().unwrap(),
                                    path_string,
                                    artifact_regex,
                                    &metadata,
                                    &gitignore,
                                )
                            } {
                            // should check size before whether it's an artifact?
                            let file_size = FileSize::new(metadata.len());
                            size.add(file_size);
                        }
                    }
                }
                // otherwise, go deeper
                else if path_type.is_dir() {
                    let dir_size = if artifacts_only && REGEX_PROJECT_DIR.is_match(path_string) {
                        read_size(
                            &path,
                            depth + 1,
                            artifact_regex,
                            excludes,
                            &gitignore,
                            false,
                            false,
                        )
                    } else {
                        read_size(
                            &path,
                            depth + 1,
                            artifact_regex,
                            excludes,
                            &gitignore,
                            with_gitignore,
                            artifacts_only,
                        )
                    };
                    size.add(dir_size);
                }
            }
            /*else {
                eprintln!(
                        "{}: ignoring symlink at {}",
                        "Warning".yellow(),
                        path.display()
                    );
                }*/
        }
    }
    // if we can't read the directory contents, figure out why
    // 1: check the path exists
    else if !in_paths.exists() {
        eprintln!(
            "{}: path '{}' does not exist.",
            "Error".red(),
            &in_paths.display()
        );
        exit(0x0001);
    }
    // 2: check the path is actually a directory
    else if !in_paths.is_dir() {
        eprintln!(
            "{}: {} is not a directory.",
            "Error".red(),
            &in_paths.display()
        );
        exit(0x0001);
    }
    // 3: otherwise, give a warning about permissions
    else {
        eprintln!(
            "{}: permission denied for directory: {}",
            "Warning".yellow(),
            &in_paths.display()
        );
    }

    size
}
/// Function to process directory contents and return a `FileTree` struct.
pub fn read_all(
    in_paths: &PathBuf,
    depth: u8,
    max_depth: Option<u8>,
    artifact_regex: Option<&Regex>,
    excludes: Option<&Regex>,
    maybe_gitignore: &Option<RegexSet>,
    with_gitignore: bool,
    artifacts_only: bool,
) -> FileTree {

    // attempt to read the .gitignore
    let mut tree = FileTree::new();
    let gitignore = if with_gitignore {
        mk_ignores(in_paths, maybe_gitignore)
    } else {
        None
    };

    // for project directories
    lazy_static! {
        static ref REGEX_PROJECT_DIR: Regex = 
            Regex::new(r"(.stack-work|dist|dist-newstyle|target|.*\.egg-info|elm-stuff)$")
            .unwrap();
    }

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            // TODO consider a filter on the iterator!
            let val = match p {
                Ok(x) => x,
                _ => {
                    eprintln!("{}: path error at {:?}.", "Error".red(), p);
                    exit(0x0001)
                }
            };
            let path = val.path();
            let (path_string, bool_loop): (&str, bool) = if let Some(x) = path.as_path().to_str() {
                let bool_loop = match excludes {
                    Some(ex) => !ex.is_match(x),
                    _ => true,
                };

                (x, bool_loop)
            } else {
                eprintln!(
                    "{}: skipping invalid unicode filepath at {:?}",
                    "Warning".yellow(),
                    path
                );
                ("", false)
            };

            // only consider path if we're not using regex excludes or if they don't match the
            // exclusion regex
            if bool_loop {

                let path_type = val.file_type().unwrap();

                // append file size/name for a file
                if path_type.is_file() {
                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = val.metadata() {
                        // faster on Windows
                        if !artifacts_only ||
                            {
                                is_artifact(
                                    val.file_name().to_str().unwrap(),
                                    path_string,
                                    artifact_regex,
                                    &metadata,
                                    &gitignore,
                                )
                            } {
                            let file_size = FileSize::new(metadata.len());
                            tree.push(path_string.to_string(), file_size, None, depth + 1, false);
                        }
                    }
                }
                // otherwise, go deeper
                else if path_type.is_dir() {
                    if let Some(d) = max_depth {
                        if depth + 1 >= d ||
                            (artifacts_only && REGEX_PROJECT_DIR.is_match(path_string))
                        {
                            let dir_size = {
                                read_size(
                                    &path,
                                    depth + 1,
                                    artifact_regex,
                                    excludes,
                                    &gitignore,
                                    with_gitignore && !REGEX_PROJECT_DIR.is_match(path_string),
                                    artifacts_only && !REGEX_PROJECT_DIR.is_match(path_string), // FIXME only compute this once
                                )
                            };
                            tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                        } else {
                            let mut subtree = read_all(
                                &path,
                                depth + 1,
                                max_depth,
                                artifact_regex,
                                excludes,
                                &gitignore,
                                with_gitignore,
                                artifacts_only,
                            );
                            let dir_size = subtree.file_size;
                            tree.push(
                                path_string.to_string(),
                                dir_size,
                                Some(&mut subtree),
                                depth + 1,
                                true,
                            );
                        }
                    }
                }
            }
        }
    /*else {
                eprintln!(
                    "{}: ignoring symlink at {}",
                    "Warning".yellow(),
                    path.display()
                );
            }*/
    }
    // if we can't read the directory contents, figure out why
    // 1: check the path exists
    else if !in_paths.exists() {
        eprintln!(
            "{}: path '{}' does not exist.",
            "Error".red(),
            &in_paths.display()
        );
        exit(0x0001);
    }
    // 2: check the path is actually a directory
    else if !in_paths.is_dir() {
        eprintln!(
            "{}: {} is not a directory.",
            "Error".red(),
            &in_paths.display()
        );
        exit(0x0001);
    }
    // 3: otherwise, give a warning about permissions
    else {
        eprintln!(
            "{}: permission denied for directory: {}",
            "Warning".yellow(),
            &in_paths.display()
        );
    }

    tree
}
