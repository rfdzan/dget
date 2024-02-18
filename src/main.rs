use clap::Parser;
use dget::{dfs, Args, IgnoreFiles, DFS};
use std::{
    collections::HashSet,
    io::{self, prelude::*, BufWriter},
};
fn main() {
    if let Err(e) = main_dfs() {
        panic!("{e}");
    }

    // let args = Args::parse();
    // let st = args.get_starting_dir();
    // let s = args.get_keywords();
    // let g = IgnoreFiles::new(st.as_path(), args.get_gitignore()).build();
    // let mut hashset = HashSet::new();
    // let mut stdout = BufWriter::new(io::stdout().lock());
    // if let Err(e) = dfs(st, s.as_str(), &g, &mut hashset, &mut stdout) {
    //     eprintln!("{e}");
    // }
}
fn main_dfs() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let args = Args::parse();
    for path in DFS::new(args) {
        // write!(stdout, "{path:?}\n)?;
    }
    Ok(())
}
