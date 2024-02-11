use clap::Parser;
use dget::{dfs, Args, IgnoreFiles};
use std::{
    collections::HashSet,
    io::{self, BufWriter},
};
fn main() {
    let args = Args::parse();
    let st = args.get_starting_dir();
    let s = args.get_keywords();
    let g = IgnoreFiles::new(st.as_path(), args.get_gitignore()).build();
    let mut hashset = HashSet::new();
    let mut stdout = BufWriter::new(io::stdout().lock());
    if let Err(e) = dfs(st, s.as_str(), &g, &mut hashset, &mut stdout) {
        eprintln!("{e}");
    }
}
