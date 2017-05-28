# file-sniffer

If you do a significant amount of programming, you'll probably end up with
several of gigabytes of artifacts scattered about. `sniff` is a tool to help you find those artifacts.

Features:
  - [x] find "fat" files and directories
  - [x] colorized output
  - [ ] find "likely build artifact" directories
  - [ ] match speed of gnu utils
    - [x] beat the crap out of the gnu utils when using regex excludes

## Installation

### Binary install

The easiest way to install is probably to download a binary from the [releases
page](https://github.com/vmchale/file-sniffer/releases).

### Cargo

If your platform doesn't have binaries, get [cargo](https://rustup.rs/). Then:

```bash
 $ cargo install file-sniffer
```

## Use

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

The benchmarks use an [ion](https://github.com/redox-os/ion) shell script and
[bench](https://github.com/Gabriel439/bench) to perform the actual benchmarks. 
