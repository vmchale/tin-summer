extern crate glob;

use std::fs;
use regex::{RegexSet, Regex};
use utils::*;
use std::path::PathBuf;
use colored::*;
use std::process::exit;
use types::*;
use std::fs::Metadata;
use error::*;
use self::glob::glob;
use std::result::Result;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;

pub fn glob_exists(s: &str) -> bool {
    glob(s).unwrap().filter_map(Result::ok).count() != 0 // ok because panic on IO Errors shouldn't happen.
}

/// Helper function to identify project directories. The heuristic is as follows:
///
/// 1. For `.stack-work`, look for a `.cabal` file or a `package.yaml` file in the parent
///    directory.
/// 2. For `target`, look for a `Cargo.toml` file in the parent directory.
/// 3. For `elm-stuff`, look for `elm-package.json` in the parent directory.
/// 4. For `build`, `dist`, look for a `.cabal`, `setup.py` or `cabal.project` file.
/// 5. For `dist-newstyle`, look for a `.cabal` or `cabal.project` file.
/// 6. For `nimcache`, look for a `.nim` file in the parent directory.
/// 6. Otherwise, if `setup.py` is in the parent directory and it ends with `.egg-info`, return
///    true.
/// 7. In all other cases, return false, but still proceed into the directory to search files by
///    extension.
pub fn is_project_dir(p: &str, name: &str) -> bool {
    // for project directories
    lazy_static! {
        static ref REGEX_PROJECT_DIR: Regex = 
            Regex::new(r"_minted|((.stack-work|.reco-work|dist|dist-newstyle|target|\.egg-info|elm-stuff)$)")
            .unwrap();
    }

    if REGEX_PROJECT_DIR.is_match(name) {
        let mut parent_path = PathBuf::from(p);
        let mut parent_string = p.to_owned();
        match name {
            ".stack-work" => {
                let mut hpack = parent_path.clone();
                parent_path.push("cabal.project");
                hpack.push("package.yaml");
                parent_string.push_str("/../*.cabal");
                parent_path.exists() || hpack.exists() || glob_exists(&parent_string)
            }
            "nimcache" => {
                parent_string.push_str("/../*.nim");
                glob_exists(&parent_string)
            }
            "target" => {
                parent_path.push("../Cargo.toml");
                parent_path.exists()
            }
            ".reco-work" => {
                parent_path.push("../main.go");
                parent_path.exists()
            }
            "elm-stuff" => {
                let mut package_path = PathBuf::from(p);
                package_path.push("../elm-package.json");
                package_path.exists()
            }
            "build" | "dist" | "dist-newstyle" => {
                let mut cabal_project = parent_path.clone();
                parent_path.push("../setup.py");
                parent_string.push_str("/../*.cabal");
                cabal_project.push("../cabal.project");
                parent_path.exists() || glob_exists(&parent_string) || cabal_project.exists()
            }
            _ => {
                let mut parent_path_latex = parent_path.clone();
                parent_path.push("../setup.py");
                parent_path_latex.push("../*.tex");
                (parent_path.exists() && str::ends_with(name, ".egg-info")) ||
                    (glob_exists(&parent_path_latex.to_string_lossy()) &&
                         str::starts_with(name, "_minted"))
            }
        }
    } else {
        false
    }

}


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
/// - `.hi`, `.hc`, `.dyn_hi`, `.dyn_o`, `.p_hi`, `.p_o`, `.prof`, `.dump-.*`, `.tix`: GHC
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
/// - `.go.v`: Go-compiled verilog
/// - `.go.teak`: Go-generated teak
#[cfg(not(target_os = "windows"))]
pub fn is_artifact(
    path_str: &str,
    full_path: &str,
    metadata: &Metadata,
    gitignore: &Option<RegexSet>,
) -> bool {

    lazy_static! {
        static ref REGEX_GITIGNORE: Regex = 
            Regex::new(r"\.(stats|conf|h|out|cache.*|dat|pc|info|\.js)$")
            .unwrap();
    }

    // otherwise, use builtin expressions
    {

        lazy_static! {
            static ref REGEX: Regex = 
                Regex::new(r"\.(a|la|lo|o|ll|keter|bc|dyn_o|d|rlib|crate|hi|hc|dyn_hi|S|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|hspec-failures|pyc|mod|go\.(v|teak)|p_hi|p_o|prof|hide-cache|\.ghc\.environment\..*-\d.\d.\d|tix|synctex\.gz)$")
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
    _: &Metadata,
    gitignore: &Option<RegexSet>,
) -> bool {

    lazy_static! {
        static ref REGEX_GITIGNORE: Regex = 
            Regex::new(r"\.(stats|conf|h|out|cache.*|dat|pc|info|\.js)$")
            .unwrap();
    }

    {

        lazy_static! {
            static ref REGEX: Regex = 
                Regex::new(r"\.(exe|a|la|o|ll|keter|bc|dyn_o|d|rlib|crate|hi|hc|dyn_hi|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|go\.(v|teak)|p_hi|p_o|prof|tix)$")
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
    excludes: Option<&Regex>,
    maybe_gitignore: &Option<RegexSet>,
    artifacts_only: bool,
) -> FileSize {

    // attempt to read the .gitignore
    let mut size = FileSize::new(0);
    let gitignore = if artifacts_only {
        mk_ignores(in_paths, maybe_gitignore)
    } else {
        None
    };

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            let val = match p {
                Ok(x) => x,
                _ => {
                    panic!("{}", Internal::IoError);
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

                let path_type = val.file_type().unwrap(); // ok because we already checked

                // append file size/name for a file
                if path_type.is_file() {
                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = val.metadata() {
                        if !artifacts_only ||
                            {
                                is_artifact(
                                    val.file_name().to_str().unwrap(), // ok because we already checked
                                    path_string,
                                    &metadata, // FIXME check metadata only when we know it matches gitignore
                                    &gitignore,
                                )
                            }
                        {
                            // should check size before whether it's an artifact?
                            let file_size = FileSize::new(metadata.len());
                            size.add(file_size);
                        }
                    }
                }
                // otherwise, go deeper
                else if path_type.is_dir() {
                    let dir_size = if artifacts_only &&
                        is_project_dir(path_string, val.file_name().to_str().unwrap())
                    {
                        read_size(&path, excludes, &gitignore, false)
                    } else {
                        read_size(&path, excludes, &gitignore, artifacts_only)
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
            "{}: path '{}' does not exist, or you do not have permission to enter.",
            "Error".red(),
            &in_paths.display()
        );
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
    excludes: Option<&Regex>,
    maybe_gitignore: &Option<RegexSet>,
    artifacts_only: bool,
) -> FileTree {

    // attempt to read the .gitignore
    let mut tree = FileTree::new();
    let gitignore = if artifacts_only {
        mk_ignores(in_paths, maybe_gitignore)
    } else {
        None
    };

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            // TODO consider a filter on the iterator!
            let val = match p {
                Ok(x) => x,
                _ => {
                    eprintln!("{}:  {:?}.", "Error".red(), p);
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

                let path_type = val.file_type().unwrap(); // ok because we already checked

                // append file size/name for a file
                if path_type.is_file() {
                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = val.metadata() {
                        // faster on Windows
                        if !artifacts_only ||
                            {
                                is_artifact(
                                    val.file_name().to_str().unwrap(), // ok because we already checked
                                    path_string,
                                    &metadata,
                                    &gitignore,
                                )
                            }
                        {
                            let file_size = FileSize::new(metadata.len());
                            tree.push(path_string.to_string(), file_size, None, depth + 1, false);
                        }
                    }
                }
                // otherwise, go deeper
                else if path_type.is_dir() {
                    if let Some(d) = max_depth {
                        if depth + 1 >= d && !artifacts_only {
                            let dir_size = {
                                read_size(&path, excludes, &gitignore, artifacts_only)
                            };
                            tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                        } else if artifacts_only &&
                                   is_project_dir(path_string, val.file_name().to_str().unwrap())
                        {
                            let dir_size = {
                                read_size(&path, excludes, &gitignore, false)
                            };
                            tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                        } else {
                            let mut subtree = read_all(
                                &path,
                                depth + 1,
                                max_depth,
                                excludes,
                                &gitignore,
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
                    } else if artifacts_only &&
                               is_project_dir(path_string, val.file_name().to_str().unwrap())
                    {
                        let dir_size = {
                            read_size(&path, excludes, &gitignore, false)
                        };
                        tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                    } else {
                        let mut subtree = read_all(
                            &path,
                            depth + 1,
                            max_depth,
                            excludes,
                            &gitignore,
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
            "{}: path '{}' does not exist, or you do not have permission to enter.",
            "Error".red(),
            &in_paths.display()
        );
    }
    // 2: check the path is actually a directory
    else if !in_paths.is_dir() {

        if artifacts_only {
            eprintln!(
                "{}: {} is not a directory; not searching for artifacts",
                "Warning".yellow(),
                &in_paths.display()
            );
        }

        if let Ok(l) = in_paths.metadata() {
            let size = l.len();
            let to_formatted = format!("{}", FileSize::new(size));
            println!("{}\t {}", &to_formatted.green(), in_paths.display());
        } else {
            panic!("{}", Internal::IoError);
        }
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

/// Function to process directory contents and return a `FileTree` struct.
pub fn read_no_excludes(
    in_paths: &PathBuf,
    _: Option<&Regex>,
    _: &Option<RegexSet>,
    _: bool,
) -> FileSize {

    // attempt to read the .gitignore
    let mut size = FileSize::new(0);

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            let val = match p {
                Ok(x) => x,
                _ => {
                    panic!("{}", Internal::IoError);
                }
            };
            // only consider path if we're not using regex excludes or
            // if they don't match the exclusion regex
            let path_type = val.file_type().unwrap(); // ok because we already checked

            // append file size/name for a file
            if path_type.is_file() {
                // if this fails, it's probably because `path` is a broken symlink
                if let Ok(metadata) = val.metadata() {
                    let file_size = FileSize::new(metadata.len());
                    size.add(file_size);
                }
            }
            // otherwise, go deeper
            else if path_type.is_dir() {
                let dir_size = {
                    let path = val.path();
                    read_no_excludes(&path, None, &None, false)
                };
                size.add(dir_size);
            }
        }
    }
    // if we can't read the directory contents, figure out why
    // 1: check the path exists
    else if !in_paths.exists() {
        eprintln!(
            "{}: path '{}' does not exist, or you do not have permission to enter.",
            "Error".red(),
            &in_paths.display()
        );
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
pub fn read_all_fast(in_paths: &PathBuf, depth: u8, max_depth: Option<u8>) -> FileTree {

    // attempt to read the .gitignore
    let mut tree = FileTree::new();

    // try to read directory contents
    if let Ok(paths) = fs::read_dir(in_paths) {

        // iterate over all the entries in the directory
        for p in paths {
            let val = match p {
                Ok(x) => x,
                _ => {
                    eprintln!("{}: unexpected failure on {:?} failed.", "Error".red(), p);
                    exit(0x0001)
                }
            };

            // only consider path if we're not using regex excludes or if they don't match the
            // exclusion regex
            let path_type = val.file_type().unwrap(); // ok because we already checked

            // append file size/name for a file
            if path_type.is_file() {
                // if this fails, it's probably because `path` is a broken symlink
                if let Ok(metadata) = val.metadata() {
                    // faster on Windows
                    {
                        let path = val.path();
                        let path_string: &str = if let Some(x) = path.as_path().to_str() {
                            x
                        } else {
                            eprintln!(
                                "{}: skipping invalid unicode filepath at {:?}",
                                "Warning".yellow(),
                                path
                            );
                            ""
                        };
                        let file_size = FileSize::new(metadata.len());
                        tree.push(path_string.to_string(), file_size, None, depth + 1, false);
                    }
                }
            }
            // otherwise, go deeper
            else if path_type.is_dir() {
                if let Some(d) = max_depth {
                    if depth + 1 >= d {
                        let path = val.path();
                        let path_string: &str = if let Some(x) = path.as_path().to_str() {
                            x
                        } else {
                            eprintln!(
                                "{}: skipping invalid unicode filepath at {:?}",
                                "Warning".yellow(),
                                path
                            );
                            ""
                        };
                        let dir_size = {
                            read_no_excludes(&path, None, &None, false)
                        };
                        tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                    } else {
                        let path = val.path();
                        let path_string: &str = if let Some(x) = path.as_path().to_str() {
                            x
                        } else {
                            eprintln!(
                                "{}: skipping invalid unicode filepath at {:?}",
                                "Warning".yellow(),
                                path
                            );
                            ""
                        };
                        let mut subtree = read_all_fast(&path, depth + 1, max_depth);
                        let dir_size = subtree.file_size;
                        tree.push(
                            path_string.to_string(),
                            dir_size,
                            Some(&mut subtree),
                            depth + 1,
                            true,
                        );
                    }
                } else {
                    let path = val.path();
                    let path_string: &str = if let Some(x) = path.as_path().to_str() {
                        x
                    } else {
                        eprintln!(
                            "{}: skipping invalid unicode filepath at {:?}",
                            "Warning".yellow(),
                            path
                        );
                        ""
                    };
                    let mut subtree = read_all_fast(&path, depth + 1, max_depth);
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
    // if we can't read the directory contents, figure out why
    // 1: check the path exists
    else if !in_paths.exists() {
        eprintln!(
            "{}: path '{}' does not exist, or you do not have permission to enter.",
            "Error".red(),
            &in_paths.display()
        );
    }
    // 2: check the path is actually a directory
    else if !in_paths.is_dir() {

        if let Ok(l) = in_paths.metadata() {
            let size = l.len();
            let to_formatted = format!("{}", FileSize::new(size));
            println!("{}\t {}", &to_formatted.green(), in_paths.display());
        } else {
            panic!("{}", Internal::IoError);
        }
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
