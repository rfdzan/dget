use clap::Parser;
use ignore::gitignore::Gitignore;
use ignore::Match;
use levenshtein::levenshtein;
use std::collections::{HashMap, HashSet};
use std::io;
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
/// Handles gitignore files
pub struct IgnoreFiles<'a> {
    current_dir: &'a Path,
    gitignore_path: Option<&'a Path>,
}
impl<'a> IgnoreFiles<'a> {
    /// Creates a new IgnoreFile.
    pub fn new(s: &'a Path, g: Option<&'a Path>) -> IgnoreFiles<'a> {
        IgnoreFiles {
            current_dir: s,
            gitignore_path: g,
        }
    }
    /// Builds a Gitignore that uses globs inside .ignore files to pattern match visited files/folder paths.
    pub fn build(&self) -> Gitignore {
        let (gitignore, _) = match self.check_for_existing_ignores() {
            IgnoreExists::No(empty_path) => Gitignore::new(empty_path),
            IgnoreExists::Yes(ignore_path) => Gitignore::new(ignore_path),
        };
        gitignore
    }
    fn check_for_existing_ignores(&self) -> IgnoreExists {
        let ignore_file_names = [".gitignore", ".ignore"];
        let mut ignore_exist = IgnoreExists::No(PathBuf::new());
        let gitignore_path = match self.gitignore_path {
            None => self.current_dir,
            Some(path) => path,
        };
        // If provided .ignore path points to a file, use it.
        if gitignore_path.is_file() {
            return IgnoreExists::Yes(gitignore_path.to_path_buf());
        }
        // If no .ignore file path is provided or it doesn't point to a file, scan the directory for one.
        // If none exist, then an empty Gitignore is also valid.
        if let Ok(read) = std::fs::read_dir(gitignore_path).map_err(|e| eprintln!("{e}")) {
            for f in read {
                let owned_path = match f {
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                    Ok(path) => path.path(),
                };
                let file_name = owned_path
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                if ignore_file_names.contains(&file_name) {
                    ignore_exist = IgnoreExists::Yes(PathBuf::from(file_name));
                    break;
                }
            }
        }
        ignore_exist
    }
}
/// Enum variants denoting the existence of a .ignore file.
#[derive(Debug)]
pub enum IgnoreExists {
    Yes(PathBuf),
    No(PathBuf),
}
/// If the edit distance as ratio is bigger than the threshold, prints the path to the terminal.
pub fn close_enough(path: &Path, to_search: &str) -> bool {
    let Some(path_name) = path.file_stem().unwrap_or_default().to_str() else {
        return false;
    };
    let Ok(edit_distance) = i32::try_from(levenshtein(path_name, to_search)) else {
        return false;
    };
    let arr = [path_name.chars().count(), to_search.chars().count()];
    let Some(max) = arr.iter().max() else {
        return false;
    };
    let Ok(max_as_i32) = i32::try_from(*max) else {
        return false;
    };
    let edit_distance_as_f64 = f64::from(edit_distance);
    let max_as_f64 = f64::from(max_as_i32);
    let ratio = (max_as_f64 - edit_distance_as_f64) / max_as_f64;
    if ratio > 0.5 {
        return true;
    }
    false
}
struct DFS {
    path: PathBuf,
    stack: Vec<PathBuf>,
    search: String,
    gitignore: Gitignore,
}
impl DFS {
    fn new(args: Args) -> DFS {
        DFS {
            path: args.get_starting_dir(),
            stack: Vec::new(),
            search: args.get_keywords(),
            gitignore: IgnoreFiles::new(args.get_starting_dir().as_path(), args.get_gitignore())
                .build(),
        }
    }
}
impl Iterator for DFS {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
pub fn dfs(
    start: PathBuf,
    search: &str,
    gitignore: &Gitignore,
    visited_vertices: &mut HashSet<PathBuf>,
    stdout: &mut dyn io::Write,
) -> io::Result<()> {
    let mut stack = Vec::with_capacity(100);
    stack.push(start.clone());

    while let Some(current_vertex) = stack.pop() {
        if !visited_vertices.insert(current_vertex.clone()) {
            continue;
        };
        let Ok(readdir) = std::fs::read_dir(current_vertex.clone()) else {
            continue;
        };
        for dir in readdir {
            let Ok(direntry) = dir else {
                continue;
            };
            match gitignore.matched(current_vertex.clone(), current_vertex.is_dir()) {
                Match::None => (),
                Match::Ignore(_) => continue,
                Match::Whitelist(_) => continue,
            }
            if close_enough(direntry.path().as_path(), search) {
                let owned_path = direntry.path();
                let disp = owned_path.display();
                writeln!(stdout, "{disp}")?;
            }
            stack.push(direntry.path());
        }
    }
    Ok(())
}
