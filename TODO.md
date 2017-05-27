- [ ] colorized output
- [ ] globs
- [ ] option to print out top *n* values
- [ ] option to recognize what "artifacts" are most likely to look like, e.g. `.a` or
  `.o` files and executable permissions.
  - [ ] flag for extra extensions of artifacts
  - [ ] flag for executables not on path
    - [ ] executable permissions but NOT for `.sh` or if it starts like a script
  - [ ] ends of filenames that "look like" hashes?
  - [ ] option to look for stuff *in* the .gitignore
  - [ ] option to show probable artifacts by number, not file size
  - [ ] also option to search hidden files first/not at all
  - [ ] regex or glob as well?
  - [ ] config file for this?? could interfere with scriptability
- [ ] default: order by "biggest"
- [ ] set depth to which to recurse intelligently, but also have a flag for
  manual.
- [ ] consider a `use_big_int` build flag for things besides desktop (up to 1YB
  is fine for most end users).
- [ ] benchmark on e.g. cabal source code + build and compare to du + rg and/or
  du + grep (+ sort)
- [ ] "fat" files, but also efficient (lazy) sorting algorithm w/ min & max.
- [ ] multiple threads for building up the vector?? idk
- [ ] when sorting, consider also dynamically picking which ones to include;
  don't rely on user to pass `-n` option by default.
