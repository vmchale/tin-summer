# file-sniffer

[![Build Status](https://travis-ci.org/vmchale/file-sniffer.svg?branch=master)](https://travis-ci.org/vmchale/file-sniffer)

If you do a significant amount of programming, you'll probably end up with
build artifacts scattered about. `sniff` is a tool to help you find those
artifacts. It's especially useful when you're writing build systems, 
because you can make sure your `clean` command gets everything.

## Installation

### Binary install

The easiest way to install for Linux or Windows is to download a binary from the [releases
page](https://github.com/vmchale/file-sniffer/releases).

### Cargo

If your platform doesn't have binaries, get [cargo](https://rustup.rs/). Then:

```bash
 $ cargo install file-sniffer
```

If you want the absolute latest version:

```bash
 $ cargo install --git https://github.com/vmchale/file-sniffer 
```

Make sure you are on nightly; if in doubt run

```bash
rustup run nightly cargo install file-sniffer
```

### Shell completions

After setting `BASH_COMPLETIONS_DIR` or `FISH_COMPLETIONS_DIR`, you can use the
`bash` or `fish` features like so:

```bash
 $ cargo install --features fish file-sniffer
```

## Use

Currently, `sniff` looks for files that either have an extension associated with artifacts
or executable files listed in the relevant `.gitignore`. 

Search current directory for directories with build artifacts:

```bash
 $ sniff ar
```

Look in `$DIR` for build artifacts and sort by size:

```bash
 $ sniff ar $DIR --sort
```

Look for artifacts or directories containing artifacts that occupy more than 1GB of disk space:


```bash
 $ sniff ar -t1G
```

### Accessibility

To turn off colorized output:

```bash
export CLICOLOR=0
```

### Features

  - [x] find "likely build artifact" directories
    - [x] use .gitignore/path to make decision
    - [ ] smart output (only first few files per dir)
  - [ ] match speed of gnu utils on traversals
    - [x] faster when finding artifacts
    - [x] faster with excludes
    - [x] faster on small directories
    - [x] beat du on large directories
  - [x] colorized output
  - [x] sort results by size

#### Languages Supported

The *intent* is to support basically anything, so if your DOC is not on the
list, feel free to open a PR or start an issue.

  - [x] Haskell (incl. GHCJS)
  - [x] rust
  - [x] julia
  - [x] python
  - [x] Elm
  - [x] nim
  - [x] Vimscript
  - [x] Idris
  - [x] FORTRAN
  - [ ] C

#### Foreign-language binaries

These are still very much works in progress; as of now errors and warnings are still in
English. Binaries will be available once things stabilize.

Français:

```bash
cargo install file-sniffer --no-default-features --feature francais # crates.io doesn't permit unicode in feature names 
```

Deutsch:

```bash
cargo install file-sniffer --no-default-features --feature deutsch
```
