# dget
Think of `grep` or `mlocate`, but dumber.
# How it works
`dget` doesn't store paths in a nightly-updated database like `mlocate`. It doesn't use regex like `grep`.  
Instead, it walks through your directories with BFS and uses Levenshtein distance to determine whether it should print the path to the terminal.
# Warning: it can be slow
Due to how simple it is, no caching etc, it can be slow (like it can take 30s or more) on directories with hundreds of thousands of files/folders.
# How to use
For help,
```
dget -h
```
To search,
```
cd your/path/here/
dget -f <your_keyword>
```
...or you can define your own path with `-s` flag.  
That's it, really.
# How to build
```
cargo build --release
```
I'm still figuring out the building toolchain(?) so that it can just exist in your PATH automatically. Bear with me.
# How to test
```
cargo test
```
Current test suite is not comprehensive enough.  

If you have suggestions for more robust testing please open an issue.
