# The Tin Summer

[![Windows build status](https://ci.appveyor.com/api/projects/status/github/vmchale/tin-summer?svg=true)](https://ci.appveyor.com/project/vmchale/tin-summer)
[![Build Status](https://travis-ci.org/vmchale/tin-summer.svg?branch=master)](https://travis-ci.org/vmchale/tin-summer)
[![](https://img.shields.io/crates/d/tin-summer.svg)](https://crates.io/crates/tin-summer)
[![](https://tokei.rs/b1/github/vmchale/tin-summer?category=code)](https://github.com/Aaronepower/tokei)

If you do a significant amount of programming, you'll probably end up with
build artifacts scattered about. `sn` is a tool to help you find those
artifacts.

`sn` is also a replacement for `du`. It has nicer
output, saner commands and defaults, and it even runs faster on big directories
thanks to multithreading.

## Installation

### Script

Enter the following in a command prompt:

```
curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git vmchale/tin-summer
```

### Binary install

If the script doesn't work for you, you can download a binary from the [releases
page](https://github.com/vmchale/tin-summer/releases).

### Cargo

If your platform doesn't have binaries, or you just want to build from source, get [cargo](https://rustup.rs/). Then:

```bash
 $ cargo install tin-summer
```

Make sure you are on nightly; otherwise

```bash
 $ rustup run nightly cargo install tin-summer
```

## Use

To list directory and file sizes for the current directory:

```
$ sn f
```

To get a list of directory sizes concurrently, excluding version control:

```
 $ sn p --exclude '\.git|\.pijul|_darcs|\.hg'
```

To get a sorted list of the 12 biggest directories in `$DIR`:

```
 $ sn sort $DIR -n12
```

To search current directory for directories with build artifacts:

```bash
 $ sn ar
```

To look for artifacts or directories containing artifacts that occupy more than 200MB of disk space:

```bash
 $ sn ar -t200M
```

### Accessibility

To turn off colorized output:

```bash
export CLICOLOR=0
```

### Comparison (or, 10 Things I Hate About du)

#### Reasons to use `du`

  * Reads disk usage, not just file sizes
  * Optionally dereferences symlinks
  * Slightly faster on small directories
  * Stable and well-supported

#### Reasons to use `sn`

  * Faster on large directories
  * Uses [regex](https://github.com/rust-lang/regex) for exclusions, making it
    dramatically faster than `du` when used with the `--exclude` flag.
  * Defaults to human-readable output
  * Colorized output
  * Nicer help via [clap](https://github.com/kbknapp/clap-rs)
  * Provides sorted output
  * Finds build artifacts
  * Reads file sizes, not disk usage
  * Extensible in Rust

#### Benchmark results

| Directory Size | Tool | Command | Time |
| -------------- | ---- | ------- | ---- |
| 600MB | sn | `sn p` | 60.74 ms |
| 600MB | sn | `sn d` | 99.92 ms |
| 600MB | du | `du -hacd2` | 88.28 ms |
| 4GB | sn | `sn p`| 185.2 ms |
| 4GB | sn | `sn d` | 271.9 ms |
| 4GB | du | `du -hacd2` | 195.5 ms |
| 700MB | sn | `sn p` | 91.05 ms |
| 700MB | sn | `sn d` | 176.3 ms |
| 700MB | du | `du -hacd2` | 153.8 ms |
| 7MB | sn | `sn p` | 19.48 ms |
| 7MB | sn | `sn d` | 12.72 ms |
| 7MB | du | `du -hacd2` | 10.13 ms |

These commands are all essentially equivalent in function, except that `sn p`
may use more threads than `sn a` or `du`. Results were obtained using Gabriel Gonzalez's [bench](https://github.com/Gabriel439/bench)
tool. You can see pretty criterion graphs
[here](http://vmchale.com/bench/tin-summer.html) or
[here](http://vmchale.com/bench/tin-summer-parallel.html).

In summary: yes, `sn` actually is faster on larger directories, but it is also
slower on small ones. I'm hoping to make it faster in the future; the current
naïve concurrency model has obvious directions for improvement.

#### Screenshots (alacritty + solarized dark)

##### The Tin Summer

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/tin-summer/master/screenshots/oskar1.png)

##### du

![Displaying a user's timeline in a terminal.](https://raw.githubusercontent.com/vmchale/tin-summer/master/screenshots/du-screenshot.png)

### Heuristic for build artifacts

Currently, `sn` looks for files that either have an extension associated with
build artifacts, or executable files that are ignored by version control. It also looks for "build
directories", like `.stack-work`, `elm-stuff`, etc. and if it finds a
configuration file like `tweet-hs.cabal`, it considers *all* their
contents to be build artifacts.

#### Languages Supported

The following is a list of languages `sn artifacts` has been tested with.
The *intent* is to support basically anything, so feel free to open a PR or start an issue.

  - [x] Haskell (incl. GHCJS)
  - [x] Rust
  - [x] Julia
  - [x] Python
  - [x] Elm
  - [x] Nim
  - [x] Vimscript
  - [x] TeX
  - [x] Idris
  - [x] FORTRAN
  - [ ] Ruby
  - [ ] C

##### Autoclean

`sn` can clean up your artifacts for you, but only for the above-indicated
languages. It is still experimental, but it has been tested and should not
delete unwanted files (though it may not clean everything it should).

### Where did the great name come from?

sn is the atomic symbol for tin. "The tin summer" is a pun on ["The Tin Drum"](https://en.wikipedia.org/wiki/The_Tin_Drum) and stuff the character "liboskar" references the book.
