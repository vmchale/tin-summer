# The Tin Summer

[![Build Status](https://travis-ci.org/vmchale/tin-drummer.svg?branch=master)](https://travis-ci.org/vmchale/tin-drummer)

If you do a significant amount of programming, you'll probably end up with
build artifacts scattered about. `sn` is a tool to help you find those
artifacts. It's especially useful when you're writing build systems, 
because you can make sure your `clean` command gets everything.

As of recently, `sn` is also a replacement for `du`. It has far nicer
output, saner commands and defaults, and it even runs faster on multicores.

## Installation

### Binary install

The easiest way to install for Linux or Windows is to download a binary from the [releases
page](https://github.com/vmchale/tin-summer/releases).

### Cargo

If your platform doesn't have binaries, get [cargo](https://rustup.rs/). Then:

```bash
 $ cargo install tin-summer
```

If you want the absolute latest version:

```bash
 $ pijul clone https://nest.pijul.com/vamchale/file-sniffer
 $ cd file-sniffer
 $ cargo install
```

Make sure you are on nightly; if in doubt

```bash
rustup run nightly cargo install tin-summer
```

## Use

Currently, `sn` looks for files that either have an extension associated with
build artifacts, as well as executable files listed in the relevant
`.gitignore`, `.ignore`, or darcs boringfile.

Search current directory for directories with build artifacts:

```bash
 $ sn ar
```

Look in `$DIR` for build artifacts and sort them by size:

```bash
 $ sn ar $DIR --sort
```

Look for artifacts or directories containing artifacts that occupy more than 200MB of disk space:

```bash
 $ sn ar -t200M
```

### Shell completions

After setting `BASH_COMPLETIONS_DIR` or `FISH_COMPLETIONS_DIR`, you can use the
`bash` or `fish` features like so:

```bash
 $ cargo install --features fish tin-summer
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
cargo install tin-summer --no-default-features --feature francais # crates.io doesn't permit unicode in feature names 
```

Deutsch:

```bash
cargo install tin-summer --no-default-features --feature deutsch
```
