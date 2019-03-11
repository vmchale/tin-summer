extern crate glob;

use self::glob::glob;
use colored::*;
use error::*;
use regex::{Regex, RegexSet};
use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;
use std::process::exit;
use std::result::Result;
use types::*;
use utils::*;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;

#[cfg(not(target_os = "windows"))]
pub fn is_executable(_: &str, metadata: &Metadata) -> bool {
    return ( metadata.permissions().mode() & 0b001001001 ) != 0;
}

#[cfg(target_os = "windows")]
pub fn is_executable(path_str: &str, _: &Metadata) -> bool {
    // executable status within "windows" is extension dependent, ie '[.](com|exe|bat|cmd)$'
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new(r"[.](com|exe|bat|cmd)$")
            .unwrap();
    }

    return REGEX.is_match(path_str);
}

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
            Regex::new(r"_minted|((\.stack-work|build|gen|cbits|ats-deps|\.atspkg|target|\.reco-work|\.cabal-sandbox|dist|\.criterion|dist-newstyle.*|target|\.egg-info|elm-stuff|\.pulp-cache|\.psc-package|output|bower_components|node_modules|\.liquid)$)")
            .unwrap();
    }

    if REGEX_PROJECT_DIR.is_match(name) {
        let mut parent_path = PathBuf::from(p);
        let mut parent_string = p.to_owned();
        match name {
            ".stack-work" => {
                let mut hpack = parent_path.clone();
                parent_path.push("../cabal.project");
                hpack.push("package.yaml");
                parent_string.push_str("/../*.cabal");
                parent_path.exists() || hpack.exists() || glob_exists(&parent_string)
            }
            "nimcache" => {
                parent_string.push_str("/../*.nim");
                glob_exists(&parent_string)
            }
            "target" => {
                let mut dhall = parent_path.clone();
                dhall.push("../atspkg.dhall");
                let mut shake = parent_path.clone();
                shake.push("../shake.hs");
                let mut elba = parent_path.clone();
                elba.push("../elba.toml");
                parent_path.push("../Cargo.toml");
                parent_path.exists() || dhall.exists() || shake.exists() || elba.exists()
            }
            ".atspkg" | "ats-deps" | "cbits" | "gen" => {
                parent_path.push("../atspkg.dhall");
                parent_path.exists()
            }
            ".criterion" => {
                parent_path.push("../Cargo.toml");
                parent_path.exists()
            }
            ".liquid" => {
                parent_string.push_str("/../*.hs");
                glob_exists(&parent_string)
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
            ".pulp-cache" | "output" | ".psc-package" => {
                let mut package_path = PathBuf::from(p);
                package_path.push("../psc-package.json");
                package_path.exists()
            }
            "build" | "dist" | ".cabal-sandbox" | "dist-newstyle" | "dist-newstyle-meta" => {
                let mut cabal_project = parent_path.clone();
                let mut parent_string_blod = parent_string.clone();
                parent_path.push("../setup.py");
                parent_string.push_str("/../*.cabal");
                cabal_project.push("../cabal.project");
                parent_string_blod.push_str("/../*.blod");
                parent_path.exists()
                    || glob_exists(&parent_string)
                    || cabal_project.exists()
                    || glob_exists(&parent_string_blod)
            }
            "bower_components" => {
                let mut package_path = PathBuf::from(p);
                package_path.push("../bower.json");
                package_path.exists()
            }
            "node_modules" => true,
            _ => {
                let mut parent_path_latex = parent_path.clone();
                parent_path.push("../setup.py");
                parent_path_latex.push("../*.tex");
                (parent_path.exists() && str::ends_with(name, ".egg-info"))
                    || (glob_exists(&parent_path_latex.to_string_lossy())
                        && str::starts_with(name, "_minted"))
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
/// - `.ll`, `.bc`: llvm
/// - `.keter`: keter
/// - `.d`: make
/// - `.c`: ATS
/// - `.rlib`, `.crate`: rust
/// - `.hi`, `.hc`, `.chi`, `.dyn_hi`, `.dyn_o`, `.p_hi`, `.p_o`, `.prof`, `.dump-.*`, `.tix`: GHC
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
/// - `.exe`: Windows executable
/// - `.sandbox.config`: Cabal sandbox configuration
/// - `.eventlog`: GHC event log
/// - `.ipa`: iOS applicative archive
/// - `.ttc`: Blodwen compiled module
pub fn is_artifact(
    path_str: &str,
    full_path: &str,
    metadata: &Metadata,
    vimtags: bool,
    gitignore: &Option<RegexSet>,
) -> bool {
    lazy_static! {
        static ref REGEX_GITIGNORE: Regex =
            Regex::new(r"\.(stats|conf|h|c|out|cache.*|dat|pc|info|ll|js)$").unwrap();
    }

    // otherwise, use builtin expressions
    {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"\.(a|i|ii|la|lo|o|keter|bc|dyn_o|d|rlib|crate|hi|hc|chi|dyn_hi|S|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|hspec-failures|pyc|vo|agdai|beam|mod|go\.(v|teak|xmldef|rewrittenast|rewrittengo|simplego|tree-(bind|eval|finish|parse))|p_hi|p_o|prof|hide-cache|ghc\.environment\..*\d.\d.\d|tix|synctex\.gz|hl|hp|sandbox\.config|exe|eventlog|ipa|ttc)$")
                .unwrap();
        }

        if REGEX.is_match(path_str) || (path_str == "tags" && vimtags) {
            true
        } else if let Some(ref x) = *gitignore {
            if is_executable(path_str, metadata) && REGEX_GITIGNORE.is_match(path_str) {
                x.is_match(full_path)
            } else {
                false
            }
        } else {
            path_str == "flxg_stats.txt"
        }
    }
}

/// Function to process directory contents and return a `FileTree` struct.
pub fn read_size(
    in_paths: &PathBuf,
    excludes: Option<&Regex>,
    maybe_gitignore: &Option<RegexSet>,
    vimtags: bool,
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
                        if !artifacts_only || {
                            is_artifact(
                                val.file_name().to_str().unwrap(), // ok because we already checked
                                path_string,
                                &metadata, // FIXME check metadata only when we know it matches gitignore
                                vimtags,
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
                    let dir_size =
                        if artifacts_only
                            && is_project_dir(path_string, val.file_name().to_str().unwrap())
                        {
                            read_size(&path, excludes, &gitignore, vimtags, false)
                        } else {
                            read_size(&path, excludes, &gitignore, vimtags, artifacts_only)
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
    vimtags: bool,
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
                        if !artifacts_only || {
                            is_artifact(
                                val.file_name().to_str().unwrap(), // ok because we already checked
                                path_string,
                                &metadata,
                                vimtags,
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
                        if depth + 1 >= d && !artifacts_only {
                            let dir_size =
                                { read_size(&path, excludes, &gitignore, vimtags, artifacts_only) };
                            tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                        } else if artifacts_only
                            && is_project_dir(path_string, val.file_name().to_str().unwrap())
                        {
                            let dir_size =
                                { read_size(&path, excludes, &gitignore, vimtags, false) };
                            tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                        } else {
                            let mut subtree = read_all(
                                &path,
                                depth + 1,
                                max_depth,
                                excludes,
                                &gitignore,
                                vimtags,
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
                    } else if artifacts_only
                        && is_project_dir(path_string, val.file_name().to_str().unwrap())
                    {
                        let dir_size = { read_size(&path, excludes, &gitignore, vimtags, false) };
                        tree.push(path_string.to_string(), dir_size, None, depth + 1, true);
                    } else {
                        let mut subtree = read_all(
                            &path,
                            depth + 1,
                            max_depth,
                            excludes,
                            &gitignore,
                            vimtags,
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
                        let dir_size = { read_no_excludes(&path, None, &None, false) };
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
