% sn (1)
% Vanessa McHale

# NAME

sn - check file size and view or clean artifacts

# SYNOPSIS

  sn [OPTION]... \<subcommand\> [DIRECTORY]... [OPTION]...

# DESCRIPTION

**sn** is a command-line tool to sniff out build artifacts. It can also
optionally clean them.

# OPTIONS

**-h**, **--help**
:   Display help

**-v**, **--version**
:   Display version information

# SUBCOMMANDS

**artifacts**, **ar**, **r**
:   Print out file sizes of build artifacts

**clean**, **c**
:   Clean build artifacts

**files**, **l**
:   Show all file sizes, not just directory sizes

**directories**, **d**, **dir**
:   Show only directory sizes

**fat**, **f**
:   Show only large directories

**parallel**, **p**
:   Same as **directories**, but in parallel.

**sort**, **o**
:   Sort results by size

**update**, **u**
:   Update to latest release

**help**
:   Display help

# EXAMPLES

```
sn p ~
```

```
sn c
```

```
sn d .
```

```
sn ar ~/work -e forks
```
