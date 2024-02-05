use clap::Parser;
pub mod dget;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = Args::cur_dir())]
    /// Starting directory
    start: String,
    #[arg(short, long)]
    /// Keywords to search for
    find: String,
}
impl Args {
    fn cur_dir() -> String {
        let pwd = match std::env::current_dir() {
            Err(e) => {
                eprintln!("{e}");
                String::with_capacity(0)
            },
            Ok(val) => {
                val
                .to_str()
                .unwrap_or_default()
                .to_string()
            },
        };
        pwd
    }
    pub fn get_starting_dir(&self) -> String {
        self.start.clone()
    }
    pub fn get_keywords(&self) -> String {
        self.find.clone()
    }
}


