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
- [x] travis ci
- [x] excludes w/ regex
  - [ ] set include paths w/ glob
- [ ] when sorting, consider also dynamically picking which ones to include if
  the user wants it.
  - [x] set threshholds even with `-n` flag
- [x] option to recognize what "artifacts" are most likely to look like, e.g. `.a` or
  `.o` files and executable permissions.
  - [x] allow `additional artifacts regex` flag
  - [ ] look in .gitignore
    - [ ] check executable permissions/binary file-ness when not on path but NOT for `.sh` or if it starts like a script (e.g. header for interpreter)
  - [x] regex
- [ ] consider a `use_big_int` build flag for things besides desktop (up to 1YB
  is fine for most end users).
- [ ] multiple threads?? feasible w/ 'fat' and 'artifacts'
- [ ] de/fr/bo translations would be nice
  - [x] stabilize interface w/ at least --exclude for artifact
- [ ] see how speed is affected with strings instead of PathBufs?
- [ ] make threshhold accept number w/ M/G tag (nom).
- [ ] use .gitignore to set recursion depth/set it intelligently
- [ ] consider using blocks like du? get du-like speeds.
- [ ] flag to print all (e.g. no max depth/etc.)
- [ ] warning if user inputs flags that make no sense
