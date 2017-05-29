- [x] colorized output
- [x] option to print out top *n* values
- [x] default: order by "biggest"
- [x] set depth to which to recurse, but also have a flag for setting it
  manually.
- [x] benchmark on e.g. cabal source code + build and compare to du + rg and/or
  du + grep (+ sort)
- [x] "fat" files, but also efficient (lazy) sorting algorithm w/ min & max.
- [x] currently panics on symlinks, which is bad
- [x] test w/ non-ascii characters in filenames
  - [x] non-ascii regex
- [x] excludes w/ regex
  - [ ] set include paths w/ glob
- [ ] when sorting, consider also dynamically picking which ones to include if
  the user wants it.
  - [x] set threshholds even with `-n` flag
- [x] option to recognize what "artifacts" are most likely to look like, e.g. `.a` or
  `.o` files and executable permissions.
  - [ ] allow `additional artifacts regex` flag
  - [ ] look in .gitignore
    - [ ] check executable permissions/binary file-ness when not on path but NOT for `.sh` or if it starts like a script (e.g. header for interpreter)
  - [ ] option to show probable artifacts by number, not file size
  - [x] regex
- [ ] consider a `use_big_int` build flag for things besides desktop (up to 1YB
  is fine for most end users).
- [ ] option to avoid hidden files
- [ ] multiple threads?? feasible w/ 'fat' and 'artifacts'
- [ ] de/fr/bo translations would be nice
- [ ] see how speed is affected with strings instead of PathBufs?
- [ ] make threshhold accept number w/ M/G tag (nom).
- [ ] use .gitignore to set recursion depth
