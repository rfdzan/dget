use clap::Parser;
use dget::{close_enough, Args, DFS};
use std::io::{self, prelude::*, BufWriter};
fn main() {
    if let Err(e) = main_dfs() {
        panic!("{e}");
    }
}
fn main_dfs() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let args = Args::parse();
    let search = args.get_keywords();
    for path in DFS::new(args) {
        if close_enough(path.as_path(), search.as_str()) {
            let disp = path.display();
            let as_string = disp.to_string();
            stdout.write(format!("{as_string}\n").as_bytes())?;
        }
        stdout.flush()?;
    }
    Ok(())
}
