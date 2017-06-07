#![feature(test)]
#![allow(match_ref_pats)]
#![allow(too_many_arguments)]
#![allow(unknown_lints)]

#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;

extern crate ignore;
extern crate regex;
extern crate colored;

#[macro_use] mod macros;
pub mod test;
pub mod types;
pub mod error;
pub mod cli_helpers;
pub mod gitignore;

pub mod prelude {


    //use std::os::unix::fs::{MetadataExt};
    use std::fs;
    use std::path::{PathBuf, Path};
    use regex::{Regex, RegexSet};
    use types::*;
    use gitignore::*;
    use colored::*;
    use std::io::prelude::*;
    use std::process::exit;
    use ignore::{WalkBuilder, WalkState};
    use std::fs::{Metadata, File};

    #[cfg(not(target_os = "windows"))]
    use std::os::unix::fs::PermissionsExt;

    pub use cli_helpers::*;
    pub use error::*;

    /// Helper function to determine whether a path points  
    ///
    /// Rules:
    /// - if the file extension of that is that of an artifact, return true
    /// - if the file is executable and included in the .gitignore, return true
    /// - return false otherwise
    ///
    /// Explanation of extensions:
    /// - `.a`, `.la`, `.o`, `.lo`, `.so.*`:
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
    pub fn is_artifact(path_str: &str, re: Option<&Regex>, metadata: &Metadata, gitignore: &Option<RegexSet>) -> bool {

        // match on the user's expression if it exists
        if let Some(r) = re {
            r.is_match(path_str)
        }

        // otherwise, use builtin expressions
        else {
            lazy_static! {
                static ref REGEX: Regex = 
                    Regex::new(r".*?\.(a|la|lo|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$") // TODO reorder regex
                    .unwrap();
            }
            lazy_static! {
                static ref REGEX_GITIGNORE: Regex = 
                    Regex::new(r".*?\.(stats|conf|h|cache.*|dat|pc|info)$")
                    .unwrap();
            }
            if REGEX.is_match(path_str) { true }
            else if let &Some(ref x) = gitignore {
                if metadata.permissions().mode() == 0o755 || REGEX_GITIGNORE.is_match(path_str)
                    { x.is_match(path_str) }
                else { false }
            }
            else { false }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn is_artifact(path_str: &str, re: Option<&Regex>, _: &Metadata, gitignore:&Option<RegexSet>) -> bool {
        if let Some(r) = re {
            r.is_match(path_str)
        }
        else {
            lazy_static! {
                static ref REGEX: Regex = 
                    Regex::new(r".*?\.(exe|a|la|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$")
                    .unwrap();
            }
            lazy_static! {
                static ref REGEX_GITIGNORE: Regex = 
                    Regex::new(r".*?\.(stats|conf|h|cache.*)$")
                    .unwrap();
            }
            if REGEX.is_match(&path_str) { true }
            
            else if let &Some(ref x) = gitignore {
                if REGEX_GITIGNORE.is_match(path_str) {
                    if x.is_match(path_str) { true } else { false }
                } else { false }
            }
            else { false }
        }
    }

    pub fn read_parallel(in_paths: &Path,
                         max_depth: Option<usize>, // optionally include a max depth to which to recurse
                         _: Option<&Regex>,
                         show_hidden: bool,
                         //maybe_gitignore: Option<RegexSet>,
                         //with_gitignore: bool,
                         artifacts_only: bool,
                         follow_symlinks: bool) -> FileSize {
        // create new walk + set file size to zero
        let mut filesize_dir = FileSize::new(0);
        let mut builder = WalkBuilder::new(in_paths);

        // set options for our walk
        builder.max_depth(max_depth);
        builder.hidden(show_hidden);
        builder.follow_links(follow_symlinks);
        builder.ignore(false);
        builder.git_ignore(false);
        builder.git_exclude(false);
        builder.git_global(false);

        // runs loop
        builder.build_parallel().run(|| Box::new(move |path| {

            if let Ok(p) = path {
                if p.path().is_file() {
                    if let Ok (f) = p.file_name().to_owned().into_string() {
                        if let Ok(metadata) = p.metadata() {
                            if !artifacts_only || is_artifact(&f, None, &metadata, &None) {
                                let file_size = FileSize::new(metadata.len());
                                filesize_dir.add(file_size);
                                let formatted = format!("{}", file_size);
                                if let Ok(s) = p.path().as_os_str().to_owned().into_string() {
                                    println!("{}\t {}", formatted.green(), s);
                                }
                                else {
                                    eprintln!("{}: ignoring invalid unicode {:?}", "Warning".yellow(), p.path()) 
                                }
                            }
                        }
                        else if follow_symlinks {
                            let s = p.path().as_os_str().to_owned().into_string().expect("OS String failed to resolve");
                            eprintln!("{}: ignoring broken symlink at {}", "Warning".yellow(), s);
                        }
                    }
                    else {
                        eprintln!("{}: ignoring invalid unicode {:?}", "Warning".yellow(), p.path()) 
                    }
                }
                // instead, consider a BTreeMap with parent directory sizes? might have to modify
                // burntsushi's crate
            }
            else if let Err(e) = path {
                eprintln!("{}: failed to get path data from:\n{:?}", "Warning".yellow(), e);
            }
            ; WalkState::Continue }));
        filesize_dir
    }

    /// Function to process directory contents and return a `FileTree` struct.
    pub fn read_all(in_paths: &PathBuf,
                          depth: u8,
                          min_bytes: Option<u64>,
                          artifact_regex: Option<&Regex>,
                          excludes: Option<&Regex>,
                          silent: bool,
                          maybe_gitignore: &Option<RegexSet>,
                          with_gitignore: bool,
                          artifacts_only: bool) -> FileTree {

        // attempt to read the .gitignore
        let mut tree = FileTree::new();
        let min_size = min_bytes.map(FileSize::new);
        let gitignore = if with_gitignore {
            if let Some(ref gitignore) = *maybe_gitignore { Some(gitignore.to_owned()) }
            else {
                let mut gitignore_path = in_paths.clone();
                gitignore_path.push(".gitignore");
                if let Ok(mut file) = File::open(gitignore_path) {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("File read failed.");
                    Some(file_contents_to_regex(&contents))
                } else { None }
            }
        } else { None };

        // try to read directory contents
        if let Ok(paths) = fs::read_dir(in_paths) {

            // iterate over all the entries in the directory
            for p in paths {
                let path = p.unwrap().path(); // TODO no unwraps; idk what this error would be though.
                let (path_string, bool_loop): (String, bool) = if let Ok(x) = path.clone().into_os_string().into_string() {
                    let bool_loop = match excludes {
                        Some(ex) => !ex.is_match(&x),
                        _ => true,
                    };

                    (x, bool_loop)
                }
                else {
                    eprintln!("{}: skipping invalid unicode filepath at {:?}", "Warning".yellow(), path); // TODO consider byte matching here?
                    ("".to_string(), false)
                };

                // only consider path if we're not using regex excludes or if they don't match the
                // exclusion regex
                if bool_loop {

                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = fs::metadata(&path) {

                        // append file size/name for a file
                        if metadata.is_file() {
                            if !artifacts_only || is_artifact(&path_string, artifact_regex, &metadata, &gitignore) { // should check size before whether it's an artifact? 
                                let file_size = FileSize::new(metadata.len());//blocks() * 512);
                                if let Some(b) = min_bytes {
                                    if file_size >= FileSize::new(b) {
                                            tree.push(path_string, file_size, None, depth + 1, min_size);
                                        }
                                    }
                                else {
                                    tree.push(path_string, file_size, None, depth + 1, min_size);
                                }
                            }
                        }

                        // otherwise, go deeper
                        
                        else if metadata.is_dir() {
                            let mut subtree = read_all(&path, depth + 1, min_bytes, artifact_regex, excludes, silent, &gitignore, with_gitignore, artifacts_only);
                            let dir_size = subtree.file_size;
                            if let Some(b) = min_bytes {
                                if dir_size >= FileSize::new(b) {
                                    tree.push(path_string, dir_size, Some(&mut subtree), depth + 1, min_size);
                                }
                            }
                            else { tree.push(path_string, dir_size, Some(&mut subtree), depth + 1, min_size); }
                        }
                    }
                    else if !silent { eprintln!("{}: ignoring symlink at {}", "Warning".yellow(), path.display()); }
                }
            }
        }

        // if we can't read the directory contents, figure out why
        // 1: check the path exists
        else if !in_paths.exists() {
            eprintln!("{}: path '{}' does not exist.", "Error".red(), &in_paths.display());
            exit(0x0001);
        }
        // 2: check the path is actually a directory
        else if !in_paths.is_dir() {
            eprintln!("{}: {} is not a directory.", "Error".red(), &in_paths.display());
            exit(0x0001);
        }
        // 3: otherwise, give a warning about permissions
        else if !silent {
            eprintln!("{}: permission denied for directory: {}", "Warning".yellow(), &in_paths.display());
        }

        tree
    }
}
