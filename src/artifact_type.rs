pub enum ArtifactType {
    Haskell,
    GHCJS,
    Rust,
    Idris,
    Julia,
    Python,
    Vimscript,
    FORTRAN,
    Assembly,
    Generic,
    LaTeX,
    Keter,
    LLVM,
    Unknown,
}

pub fn match_extension(file_name: &str, full_path: &str) -> ArtifactType {

    lazy_static! {
        static ref REGEX_HASKELL: Regex = 
            Regex::new(r".*?\.(dyn_o|out|d|hi|dyn_hi|dump-.*|p_hi|p_o|prof|tix)$")
            .unwrap();
    }

    lazy_static! {
        static ref REGEX_HASKELL: Regex = 
            Regex::new(r".*?\.(ibc)$")
            .unwrap();
    }

    lazy_static! {
        static ref REGEX_HASKELL_FULL: Regex = 
            Regex::new(r"\.stack-work|dist-newstyle")
            .unwrap();
    }

    if REGEX_HASKELL.is_match(file_name) {
        ArtifactType::Haskell
    }
    else if REGEX_IDRIS.is_match(file_name) {
        ArtifactType::Idris
    }
    // only check full regex when we've exhausted the extensions, since they take longer
    else if REGEX_HASKELL_FULL.is_match(file_name) {
        ArtifactType::Haskell
    }
    // r".*?\.(a|la|lo|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|S|jsexe|webapp|js\.externs|ibc|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|jld|ji|js_o|so.*|dump-.*|vmb|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$"
}
