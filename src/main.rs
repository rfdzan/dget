use clap::Parser;
use dget::{dfs, Args, IgnoreFiles};
use std::{
    collections::HashMap,
    io::{self, BufWriter},
};
fn main() {
    let args = Args::parse();
    let st = args.get_starting_dir();
    let s = args.get_keywords();
    let g = IgnoreFiles::new(st.as_path(), args.get_gitignore()).build();
    let mut hashmap = HashMap::new();
    let mut stdout = BufWriter::new(io::stdout().lock());
    if let Err(e) = dfs(st, s.as_str(), &g, &mut hashmap, &mut stdout) {
        eprintln!("{e}");
    }
}
