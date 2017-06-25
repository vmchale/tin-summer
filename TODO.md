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
- [x] de/fr/bo translations would be nice
  - [x] stabilize interface w/ at least --exclude for artifact
- [x] fix .gitignore parser
- [x] fix darcs parser
- [x] make a pun about the tin drum?
- [x] still use gitignore even w/ user-supplied regex
- [x] print by significant figures

# Bugs

- [ ] flag -fd0 should work
- [ ] custom regex should override the "project dirs" regex?
- [ ] fix `unwrap()`s and `expect()`s.

# UI/Ergonomics

- [x] excludes w/ regex
  - [x] fix bugs w/ excludes & overzealous use of .gitignores
  - [ ] set include paths w/ glob
- [ ] flag to print all (e.g. no max depth/etc.)
- [ ] warn users when contradictory flags are issued
- [ ] don't display tons of files from the same directory
- [ ] let it run on a single file
- [ ] check for ignore files in the parent dir too.
- [ ] remove all expect()/unwrap() values

# Features

- [ ] flag to follow symlinks
  - [x] ignore symlinks by default
- [ ] improve ergonomics (and possibly speed) by guessing language of project
  directory
- [ ] cool feature: highlight extensions
  - [ ] color-coded by language?
- [ ] global gitignore
  - [ ] ~/.darcs/boring
  - [x] darcs boring file?
  - [x] pijul ignore
  - [ ] .hginore
- [x] regex should only match against file name, not full path
- [x] replace du
- [ ] add option to *only* search ignored files
- [ ] autoclean option
  - [ ] haskell
  - [ ] rust
  - [ ] idris
  - [ ] elm
  - [ ] python

# Performance

- [ ] parity with du without threading
  - [ ] test walkdir crate
  - [ ] dual-threading to pop off values?
- [ ] print directories immediately rather than adding them to a vector?
  - [ ] all
  - [ ] fat
  - [ ] ar

# Parallel traversals

- [ ] determine whether to use multiple threads automatically
- [ ] feed results into a queue and pop them off in another thread
- [ ] make generic structure for a traversal in parallel that respects necessary
  features
  - [ ] global ignores
  - [ ] ignore files
    - [ ] .ignore
    - [ ] pijul .ignore
    - [ ] darcs boring file
    - [ ] .hginore
  - [ ] excludes
- [ ] look at tokei/rayon??

# French/German

- [ ] errors should be translated to french/german as well.
  - [ ] make an error type & use that to organize things
- [x] change french/german binary name
  - [ ] upsteam PR to clap-rs?
- [ ] fix build.rs
