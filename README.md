![img](https://i.imgur.com/yk1iwSr.png)
Think of `mlocate`, but dumber.
# How it works
`dget` doesn't store paths in a nightly-updated database like `mlocate`. It doesn't use regex like `grep`.  
Instead, it walks through your directories with BFS and uses Levenshtein distance to determine whether it should print the path to the terminal.

If `dget` finds an `ignore` file (`.gitignore` or `.ignore`) in the same directory you're searching in, it will use it. You can also add your own custom ignore file.
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

Add a custom ignore file,
```
dget -f <your_keyword> -g /path/to/your/ignore_file.ignore
```
...`ignore` files can be anything that is writeable by `gedit`. but just use `.ignore`/`.gitignore` for convention.

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
