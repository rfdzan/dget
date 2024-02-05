use clap::Parser;
use dget::{dget::dget_main, Args};
fn main() {
    let args = Args::parse();
    dget_main(
        args.get_starting_dir().as_str(),
        args.get_keywords().as_str(),
    );
}
