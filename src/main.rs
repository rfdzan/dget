use clap::Parser;
use dget::{dfs, dget::dget_main, Args, IgnoreFiles};
use std::{
    collections::HashMap,
    io::{self, BufWriter},
};
fn main() {
    let args = Args::parse();
    // dget_main(
    //     args.get_starting_dir(),
    //     args.get_keywords().as_str(),
    //     args.get_gitignore(),
    //     &mut io::stdout().lock(),
    // );
    let st = args.get_starting_dir();
    let s = args.get_keywords();
    let g = IgnoreFiles::new(st.as_path(), args.get_gitignore()).build();
    let mut hashmap = HashMap::new();
    let mut stdout = BufWriter::new(io::stdout().lock());
    if let Err(e) = dfs(st, s.as_str(), &g, &mut hashmap, &mut stdout) {
        eprintln!("{e}");
    }
}
