# Done 

- [x] colorized output
- [x] option to print out top *n* values
- [x] default: order by "biggest"
- [x] set depth to which to recurse, but also have a flag for setting it
  manually.
- [x] benchmark on e.g. cabal source code + build and compare to du + rg and/or
  du + grep (+ sort)
- [x] "fat" files, but also efficient (lazy) sorting algorithm w/ min & max.
- [x] currently panics on symlinks, which is bad
- [x] output error messages to stderr
- [x] add tests
- [x] flag to print files
- [x] feature for bash completions
- [x] make threshhold accept number w/ M/G tag (nom).
- [x] test w/ non-ascii characters in filenames
  - [x] non-ascii regex
- [x] travis ci
- [x] set threshholds even with `-n` flag
- [x] option to recognize what "artifacts" are most likely to look like, e.g. `.a` or
  `.o` files and executable permissions.
  - [x] allow `additional artifacts regex` flag
  - [x] look in .gitignore
    - [x] check executable permissions/binary file-ness
  - [x] regex
- [x] strip out machinery for `with_gitignore` and `artifact_regex`
- [x] de/fr/bo translations would be nice
  - [x] stabilize interface w/ at least --exclude for artifact
- [x] fix .gitignore parser
- [x] fix darcs parser
- [x] make a pun about the tin drum?
- [x] still use gitignore even w/ user-supplied regex
- [x] print by significant figures
- [x] `-d1` should work with `parallel`
- [x] flag to print all (e.g. no max depth/etc.)
- [x] excludes w/ regex
  - [x] fix bugs w/ excludes & overzealous use of .gitignores
  - [x] multiple included paths
- [x] let it run on a single file
- [x] don't call `is_project_dir()` three times
- [x] improve ergonomics (and speed) by guessing language of project
  directory
- [x] subcommand to print files by default
  - [x] organize the subcommands in a saner manner
- [x] threshold flag should apply to the whole thing, so `sn fat` should not
  print main dir in such cases.
- [x] bug w/ `sort` & `-d0`.
- [x] when using `--all` with `ar`, it should not recurse arbitrarily far.
- [x] when `--all` is used with `sort`, it *should* recurse arbitrarily far.
  - [x] and `sn ar --sort` as well (currently it just explodes)

# Bugs

- [ ] `-j1` should work

# UI/Ergonomics

- [ ] silent flag to ignore warnings?
- [ ] get rid of walkdir dependency
- [ ] vim plugin
- [ ] symlinks!!
- [ ] flag to fail on nonrecoverable failures
  - [ ] fail without breaking when we can.
- [ ] get block sizes not file lengths?
- [ ] mention [the bug tracker?](https://nest.pijul.com/vamchale/tin-summer/issues) for internal errors.

# Features

- [ ] cool feature: highlight extensions
  - [ ] color-coded by language?
- [ ] global gitignore
  - [ ] check for ignore files in the parent dir too.
  - [ ] ~/.darcs/boring
  - [x] darcs boring file?
  - [x] pijul ignore
  - [ ] .hginore
- [x] regex should only match against file name, not full path
- [x] replace du
- [ ] add option to *only* search ignored files for `ar` (?)
- [ ] autoclean option
  - [ ] haskell
  - [ ] rust
  - [ ] idris
  - [ ] elm
  - [ ] python

# Tests

- [ ] test w/ actual file tree & `assert\_eq!()`
- [ ] test parsing of *all* ignore files

# Code maintenance

- [ ] make `read_all()` take a struct.

# Performance

- [ ] parity with du without threading
  - [ ] separate functions based on whether we are using exclusions/artifacts or
    not
- [ ] print directories immediately rather than adding them to a vector?
  MsQueue?
  - [ ] all
  - [ ] fat
  - [ ] ar

# Parallel traversals

- [ ] determine whether to use multiple threads automatically
- [ ] More intelligent concurrency (two levels down?)
- [x] feed results into a queue and pop them off in another thread
- [x] make generic structure for a traversal in parallel that respects necessary
  features
  - [ ] global ignores
  - [ ] ignore files
    - [ ] .ignore
    - [ ] pijul .ignore
    - [ ] darcs boring file
    - [ ] .hginore
  - [x] excludes
- [ ] look at tokei/rayon for globbed paths.

# French/German

- [ ] errors should be translated to french/german as well.
  - [ ] make an error type & use that to organize things
- [x] change french/german binary name
  - [ ] upsteam PR to clap-rs?
- [x] fix build.rs
