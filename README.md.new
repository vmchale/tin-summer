# file-sniffer

If you do a significant amount of programming, you'll probably end up with
several of gigabytes of artifacts scattered about. `sniff` is a tool to help you find those artifacts.

`sniff` can be used to find "fat" directories, but it's also smart. It will look
for directories with `.a` files and `.o` files, look *in* directories
  specified by a `.gitignore`, and check permissions/PATHs.

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

Search current directory for directories with build artifacts.

```bash
 $ sniff artifacts
```

Look for subdirectories/files that consume the most disk space.

```bash
 $ sniff fat dir
```
