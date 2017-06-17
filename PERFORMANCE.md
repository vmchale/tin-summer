Places where `sniff` is faster than `du`:

  * Traversing large directories
  * Traversing a directory and excluding certain files by regex

Places where `sniff` is slower than `du`:

  * Traversing anything *but* large directories
  * Traversing large directories without multiple cores

# Design Principles

`sniff` is designed to be find junk on your hard drive. It is also designed to
be used by the end user, rather than parsed by another program or script. With
those constraints in mind, we knock out a couple benchmarks.

## Print speed

We don't really care how fast `sniff` prints stuff out, because it shouldn't
print very much out! There relatively few instances where a user would want more
than 20 lines of output from a shell command. So we should confine ourselves to
commands whose output is user-readable.

## Convergence

Absolutes matter. Shaving 5% off of a 200ms run is more important than shaving
5% off a 20ms run. So we only care about benchmarks when it is
either

  A. Egregiously underperforming `du` and takes over 20ms, or
  B. Running over a large directory

This means that we can add things like parallel directory traversals.
