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
- [x] excludes w/ regex
  - [ ] set include paths w/ glob
- [x] make threshhold accept number w/ M/G tag (nom).
- [ ] use .gitignore to set recursion depth/set it intelligently
- [ ] flag to print all (e.g. no max depth/etc.)
- [ ] flag to follow symlinks
- [ ] output machine-generated config files as well? in e.g. yellow
