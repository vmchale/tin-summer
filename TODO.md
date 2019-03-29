# Bugs

- [ ] `sn p` hangs indefinitely on dir that doesn't exist or single file
- [ ] `.h` files are not artifacts in `clash-gch` repo but are treated as such
- [ ] `sn c --exclude` should apply to files as well as directories

# UI/Ergonomics

- [ ] silent flag to ignore warnings?
- [ ] vim plugin
- [ ] symlinks!!
- [ ] flag to fail on nonrecoverable failures
  - [ ] fail without breaking when we can.
- [ ] get block sizes not file lengths?

# Features

- [ ] `.cmi`, `.cmo`, `.cmx`
- [ ] `.bbl`, `.blg`
- [ ] `.chi`, `.chs.h`
- [ ] `.hie`
- [ ] Remove `.pyre` directory
- [ ] Remove `.history` from Dhall REPL
- [ ] show raw sizes
- [ ] make a `.deb` crate (CI)
- [ ] optionally remove docs for elm/Idris while cleaning
- [ ] parse Makefiles (clean)
- [ ] cool feature: highlight extensions
  - [ ] color-coded by language?
- [ ] global gitignore
  - [ ] check for ignore files in the parent dir too.
  - [ ] ~/.darcs/boring
  - [ ] .hginore
- [x] autoclean option
  - [ ] OCaml
- [ ] .deb should include completions and manpages.
- [ ] `.pmc` files

# Tests

- [ ] test w/ actual file tree & `assert\_eq!()`
- [ ] test parsing of *all* ignore files

# Code maintenance

- [ ] make `read_all()` take a struct.

# Performance

- [ ] parity with du without threading
- [ ] print directories immediately rather than adding them to a vector?
  MsQueue?

# Parallel traversals

- [ ] determine whether to use multiple threads automatically
- [ ] More intelligent concurrency (two levels down?)
- [x] make generic structure for a traversal in parallel that respects necessary
  features
  - [ ] global ignores
  - [ ] ignore files
    - [ ] .ignore
    - [ ] pijul .ignore
    - [ ] darcs boring file
    - [ ] .hginore
- [ ] look at rayon for globbed paths.

# French/German

- [ ] errors should be translated to french/german as well.
  - [ ] make an error type & use that to organize things
- [x] change french/german binary name
  - [ ] upsteam PR to clap-rs?
