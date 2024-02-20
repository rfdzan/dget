use clap::Parser;
use dget::{close_enough, Args, DGET};
use std::io::{self, prelude::*};
fn main() {
    if let Err(e) = dget() {
        panic!("{e}");
    }
}
fn dget() -> io::Result<()> {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    let args = Args::parse();
    let search = args.get_keywords();
    for path in DGET::new(args).filter(|path| close_enough(path.as_path(), search.as_str())) {
        let disp = path.display();
        let as_string = disp.to_string();
        stdout.write(format!("{as_string}\n").as_bytes())?;
        stdout.flush()?;
    }
    Ok(())
}
