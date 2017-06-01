 #![feature(test)] 

#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;

extern crate regex;
extern crate colored;

pub mod test;
pub mod types;
pub mod error;
pub mod cli_helpers;
pub mod gitignore;

pub mod prelude {

    use std::fs;
    use std::path::PathBuf;
    use regex::{Regex, RegexSet};
    use types::*;
    use colored::*;
    use gitignore::*;
    use std::io::prelude::*;
    use std::process::exit;
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
    /// - if the file looks like a configuration file and is in the .gitignore, return true
    /// (\.cache.*, \.conf
    /// - return false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use libsniff::prelude::*;
    /// use std::path::PathBuf;
    ///
    /// let path_buf: PathBuf = PathBuf::from("lib.so");
    /// assert_eq!(is_artifact(&path_buf, None), true);
    /// ```
    #[cfg(not(target_os = "windows"))]
    pub fn is_artifact(p: &PathBuf, re: Option<&Regex>, metadata: Metadata, gitignore: &Option<RegexSet>) -> bool {

        // get path as a string so we can match against it
        let path_str = p.clone().into_os_string().into_string().expect("OS string invalid.");

        // match on the user's expression if it exists
        if let Some(r) = re {
            r.is_match(&path_str)
        }

        // otherwise, use builtin expressions
        else {
            lazy_static! {
                static ref REGEX: Regex = 
                    Regex::new(r".*?\.(a|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o|so.*|dump-.*|vba|crx)$")
                    .unwrap();
            }
            lazy_static! {
                static ref REGEX_GITIGNORE: Regex = 
                    Regex::new(r".*?\.(a|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o|so.*|dump-.*|vba|crx|cache|conf|h|cache.*|lock)$")
                    .unwrap();
            }
            if REGEX.is_match(&path_str) { true }
            
            else if let &Some(ref x) = gitignore {
                if x.is_match(&path_str) {
                    metadata.permissions().mode() == 0o755 || REGEX_GITIGNORE.is_match(&path_str)
                } else { false }
            }
            else { false }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn is_artifact(p: &PathBuf, re: Option<&Regex>, _: Metadata, gitignore:&Option<RegexSet>) -> bool {
        let path_str = &p.into_os_string().into_string().expect("OS String invalid.");
        if let Some(r) = re {
            r.is_match(path_str)
        }
        else {
            lazy_static! {
                static ref REGEX: Regex = 
                    Regex::new(r".*?\.(exe|a|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o|so.*|dump-.*|vba|crx)$")
                    .unwrap();
            }
            lazy_static! {
                static ref REGEX_GITIGNORE: Regex = 
                    Regex::new(r".*?\.(exe|a|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o|so.*|dump-.*|vba|crx|cache|conf|h|cache.*|lock)$")
                    .unwrap();
            }
            if REGEX.is_match(&path_str) { true }
            
            else if let &Some(ref x) = gitignore {
                if REGEX_GITIGNORE.is_match(&path_str) {
                    if x.is_match(&path_str) { true } else { false }
                } else { false }
            }
            else { false }
        }
    }

    // how depth/recursion SHOULD work for artifacts: if e.g. .stack-work/ has *multiple* subdirs
    // with artifacts, then list it in place of all of them. Basically find "root nodes" of these
    // places - hard but potentially very nice? look in .gitignore?
    /// Function to process directory contents and return a `FileTree` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsniff::prelude::*;
    /// use std::path::PathBuf;
    /// 
    /// let path = PathBuf::from("src");
    /// let file_tree = read_all(&path, 2, None, None, None, false, true);
    /// ```
    pub fn read_all(in_paths: &PathBuf,
                          depth: u8,
                          min_bytes: Option<u64>,
                          artifact_regex: Option<&Regex>,
                          excludes: Option<&Regex>,
                          silent: bool,
                          maybe_gitignore: &Option<RegexSet>,
                          artifacts_only: bool) -> FileTree {

        // attempt to read the .gitignore
        let mut tree = FileTree::new();
        let min_size = min_bytes.map(FileSize::new);
        let gitignore = if let &Some(ref gitignore) = maybe_gitignore { Some(gitignore.to_owned()) }
            else {
                let mut gitignore_path = in_paths.clone();
                gitignore_path.push(".gitignore");
                if let Ok(mut file) = File::open(gitignore_path) {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("File read failed. Is your system configured to use unix?");
                    Some(file_contents_to_regex(&contents))
                } else { None }
            };

        // try to read directory contents
        if let Ok(paths) = fs::read_dir(in_paths) {

            // iterate over all the entries in the directory
            for p in paths {
                let path = p.unwrap().path(); // TODO no unwraps; idk what this error would be though.
                let path_string = path.clone().into_os_string().into_string().expect("OS String invalid."); // TODO nicer error message, mention windows/utf-8?
                let bool_loop = match excludes {
                    Some(ex) => !ex.is_match(&path_string),
                    _ => true,
                };

                // only consider path if we're not using regex excludes or if they don't match the
                // exclusion regex
                if bool_loop {

                    // if this fails, it's probably because `path` is a broken symlink
                    if let Ok(metadata) = fs::metadata(&path) {

                        // append file size/name for a file
                        if metadata.is_file() {
                            if !artifacts_only || is_artifact(&path, artifact_regex, metadata.clone(), &gitignore) {
                                let file_size = FileSize::new(metadata.len());
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
                            let mut subtree = read_all(&path, depth + 1, min_bytes, artifact_regex, excludes, silent, &gitignore, artifacts_only);
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
            eprintln!("{}: path '{}' does not exist.", "Error".red(), &in_paths.display()); // FIXME check it is a directory too
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
