% sn (1)
% Vanessa McHale

# NAME

sn - check file size and view or clean artifacts

# SYNOPSIS

  sn parallel \<dir\>

  sn artifacts \<dir\>

  sn clean \<dir\>

# DESCRIPTION

**sn** is a command-line tool to sniff out build artifacts. It can also
optionally clean them.

# OPTIONS

**-h**, **--help**
:   Display help

**-v**, **--version**
:   Display version information

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
sn ar -e forks
```
