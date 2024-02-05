# dget
Think of `grep` or `mlocate`, but dumber.
# How it works
`dget` doesn't store paths in a nightly-updated database like `mlocate`. It doesn't use regex like `grep`.  
Instead, it walks through your directories with BFS and uses Levenshtein distance to determine whether it should print the path to the terminal.
# Warning
Due to how simple it is, no caching etc, it can be slow (like it can take 30s or more) on directories with hundreds of thousands of files/folders.
# How to use
For help,
```
dget -h
```
To search,
```
dget -f <your_keyword>
```
That's it, really.
