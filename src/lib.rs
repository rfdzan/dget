use clap::Parser;
use ignore::gitignore::Gitignore;
use std::path::{Path, PathBuf};
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
    /// Custom gitignore file path
    #[arg(short, long, default_value_t=Default::default())]
    gitignore: String,
}
impl Args {
    fn cur_dir() -> String {
        let pwd = match std::env::current_dir() {
            Err(e) => {
                eprintln!("{e}");
                String::with_capacity(0)
            }
            Ok(val) => val.to_str().unwrap_or_default().to_string(),
        };
        pwd
    }
    pub fn get_starting_dir(&self) -> PathBuf {
        PathBuf::from(self.start.as_str())
    }
    pub fn get_keywords(&self) -> String {
        self.find.clone()
    }
    pub fn get_gitignore(&self) -> Option<&Path> {
        if !self.gitignore.is_empty() {
            if !PathBuf::from(self.gitignore.as_str()).exists() {
                eprintln!("ignore file {} does not exist", self.gitignore);
                std::process::exit(0);
            }
            Some(Path::new(self.gitignore.as_str()))
        } else {
            None
        }
    }
}
pub struct IgnoreFiles<'a> {
    current_dir: &'a Path,
    gitignore_path: Option<&'a Path>,
}
impl<'a> IgnoreFiles<'a> {
    pub fn new(s: &'a Path, g: Option<&'a Path>) -> IgnoreFiles<'a> {
        IgnoreFiles {
            current_dir: s,
            gitignore_path: g,
        }
    }
    pub fn build(&self) -> Gitignore {
        let (gitignore, _) = match self.check_for_existing_ignores() {
            IgnoreExists::No(empty_path) => Gitignore::new(empty_path),
            IgnoreExists::Yes(ignore_path) => Gitignore::new(ignore_path),
        };
        gitignore
    }
    fn check_for_existing_ignores(&self) -> IgnoreExists {
        let ignore_files = [".gitignore", ".ignore"];
        let mut ignore_exist = IgnoreExists::No(PathBuf::new());
        let gitignore_path = match self.gitignore_path {
            None => self.current_dir,
            Some(path) => path,
        };
        if gitignore_path.is_file() {
            return IgnoreExists::Yes(gitignore_path.to_path_buf());
        }
        let read_dir = std::fs::read_dir(gitignore_path);

        match read_dir {
            Err(e) => eprintln!("{e}"),
            Ok(read) => {
                for f in read {
                    match f {
                        Err(e) => eprintln!("{e}"),
                        Ok(path) => {
                            let owned_path = path.path();
                            let path = owned_path
                                .file_stem()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default();
                            if ignore_files.contains(&path) {
                                ignore_exist = IgnoreExists::Yes(PathBuf::from(path));
                                break;
                            }
                        }
                    }
                }
            }
        }
        ignore_exist
    }
}

#[derive(Debug)]
pub enum IgnoreExists {
    Yes(PathBuf),
    No(PathBuf),
}
