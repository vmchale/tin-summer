# Thoughts on performance and benchmarking

As it stands, there aren't many comparable tools, so benchmarking is pretty
hard. I'm not sure how useful benchmarking even is: the primary
factor when running `sniff ar` is going to be the file system's page
cache<sup>1</sup>, and unlike [ripgrep](http://blog.burntsushi.net/ripgrep/), the default use case
isn't necessarily "repeatedly search the same directory or its subdirectories".

And, realistically, you probably wouldn't actually type this in to find
artifacts<sup>2</sup>:

```bash
 $ du -hac /home/vanessa/programming/haskell/forks/cabal | grep -P '(.*?\.(a|la|lo|o|ll|keter|bc|dyn_o|out|d|rlib|crate|min\.js|hi|dyn_hi|jsexe|webapp|js\.externs|toc|aux|fdb_latexmk|fls|egg-info|whl|js_a|js_hi|js_o|so.*|dump-.*|vba|crx|orig|elmo|elmi|pyc|mod|p_hi|p_o|prof|tix)$|total)'
```

## Getting solid test data

Basically: results vary depending on my input data. Running on a
rust build directory (167M), `sniff ar` is consistently fastest. Running on a haskell build
directory, `du` piped into `rg` is consistently fastest, but of course it has far fewer features.

### Directory traversals

Benchmarking directory traversals makes sense because `du -had2` being faster
than `sniff all` means that `sniff` has room to improve. That being said, `sniff`
is on the same order of magnitude, which is auspicious.

### File extensions

This is where `sniff` pulls ahead - but only sometimes. For the rust build,
`sniff` was considerably faster. For the Haskell build, it was slightly slower
than `du` combined with `rg`, and considerably faster than `du` with `grep`. It
also gave considerably cleaner output:

### Excludes

`du` handles excludes *really* poorly. I'm not sure why, but that's one area
where `sniff` doesn't need 

## Steps forward

Long-term, I want to make `sniff` a viable replacement for `du`3. That's going to
take awhile.  

In the meantime, I think parallel directory traversals are probably
the most important factor when it comes to performance. This will likely involve
forking BurntSushi's [ignore](https://docs.rs/ignore/0.2.0/ignore/) crate.

<sup>1: Running `sniff ar` on my projects took 7min34s the first time and then 736 ms thereafter.</sup>

<sup>2: Admittedly, you might put it in a bash script, but then you would have to decide whether to pass command-line arguments to `du` or to `grep`.</sup>
