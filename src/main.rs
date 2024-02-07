use clap::Parser;
use dget::{dget::dget_main, Args};
use std::io;
fn main() {
    let args = Args::parse();
    dget_main(
        args.get_starting_dir(),
        args.get_keywords().as_str(),
        args.get_gitignore(),
        &mut io::stdout().lock()
    );
}
