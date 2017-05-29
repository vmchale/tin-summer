# file-sniffer

[![Build Status](https://travis-ci.org/vmchale/file-sniffer.svg?branch=master)](https://travis-ci.org/vmchale/file-sniffer)

If you do a significant amount of programming, you'll probably end up with
build artifacts scattered about. `sniff` is a tool to help you find those
artifacts. It's also a useful aid when you're writing build systems 
(because you can make sure your `clean` command actually cleans everything).

`sniff` can be used to find big files/directories, but so can nimble use of the
gnu coreutils. 
What makes `sniff` special is that it can look *only* at files that are likely
build artifacts e.g. files with `.a` or `.o` extensions.

Features:
  - [x] find "fat" files and directories
  - [x] colorized output
  - [x] find "likely build artifact" directories
    - [x] use .gitignore/path to make decision
  - [ ] match speed of gnu utils
    - [x] beat the crap out of the gnu utils when using regex excludes

## Installation

### Binary install

The easiest way to install for Linux or Windows is to download a binary from the [releases
page](https://github.com/vmchale/file-sniffer/releases).

### Cargo

If your platform doesn't have binaries, get [cargo](https://rustup.rs/). Then:

```bash
 $ cargo install file-sniffer
```

## Use

Search current directory for directories with build artifacts:

```bash
 $ sniff artifacts
```

Look for subdirectories/files that consume the most disk space:

```bash
 $ sniff sort dir
```

Look for in the current directory for directories/files that occupy more than 1GB of disk space:


```bash
 $ sniff fat --threshhold G
```

### Accessibility

To turn off colorized output:

```bash
export CLICOLOR=0
```

### Benchmarks

#### Replicating the benchmarks

The benchmarks use an [ion](https://github.com/redox-os/ion) shell script and
[bench](https://github.com/Gabriel439/bench) to perform the actual benchmarks.
Unfortunately, I'm not sure how meaningful they are, because as far as I know,
there aren't any comparable tools.

I ran them on the built source of cabal, but you can use any directory to
benchmark them with:

```bash
./ion/bench --path $PATH_TO_DIR
```
