use dget::{dget::dget_main, Args};
use clap::Parser;
fn main() {
    let args = Args::parse();
    dget_main(args.get_starting_dir().as_str(), args.get_keywords().as_str());
}
