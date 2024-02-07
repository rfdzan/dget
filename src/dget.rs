use crate::IgnoreFiles;
use ignore::Match;
use levenshtein::levenshtein;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::path::{Path, PathBuf};

pub fn dget_main(
    start: PathBuf,
    to_search: &str,
    gitignore: Option<&Path>,
    stdout: &mut dyn io::Write,
) {
    if let Err(e) = dget(start, to_search, gitignore, stdout) {
        eprintln!("{e}")
    }
}
fn close_enough(path: &Path, to_search: &str) -> bool {
    match path.file_stem().unwrap_or_default().to_str() {
        None => false,
        Some(path_name) => {
            let edit_distance = match i32::try_from(levenshtein(path_name, to_search)) {
                Err(_) => return false,
                Ok(val) => val,
            };
            let arr = [path_name.chars().count(), to_search.chars().count()];
            match arr.iter().max() {
                None => false,
                Some(max) => {
                    let max_as_i32 = match i32::try_from(*max) {
                        Err(_) => return false,
                        Ok(val) => val,
                    };
                    let edit_distance_as_f64 = f64::from(edit_distance);
                    let max_as_f64 = f64::from(max_as_i32);
                    let ratio = (max_as_f64 - edit_distance_as_f64) / max_as_f64;
                    if ratio > 0.5 {
                        return true;
                    }
                    false
                }
            }
        }
    }
}
fn dget(
    start: PathBuf,
    to_search: &str,
    gitignore: Option<&Path>,
    stdout: &mut dyn io::Write,
) -> io::Result<()> {
    let gitignore = IgnoreFiles::new(start.as_path(), gitignore).build();
    let mut visited_vertices = HashMap::with_capacity(1000);
    let mut deque = VecDeque::with_capacity(1000);
    visited_vertices.insert(start.clone(), false);
    deque.push_back(start);

    while !deque.is_empty() {
        let current_node = deque.pop_front();
        if let Some(path) = current_node {
            match gitignore.matched(path.clone(), path.is_dir()) {
                Match::None => (),
                Match::Ignore(_) => continue,
                Match::Whitelist(_) => continue,
            }
            if let Some(true) = visited_vertices.get(&path) {
                continue;
            }
            if close_enough(path.as_path(), to_search) {
                let disp = path.display();
                writeln!(stdout, "{disp}")?;
            }
            if path.is_file() {
                continue;
            }
            if path.is_symlink() {
                visited_vertices.insert(path.clone(), true);
                deque.push_back(path);
                continue;
            }
            visited_vertices.insert(path.clone(), true);
            match std::fs::read_dir(&path) {
                Err(_) => {
                    continue;
                }
                Ok(nodes) => {
                    for node in nodes {
                        match node {
                            Err(_) => {
                                continue;
                            }
                            Ok(direntry) => {
                                let node_pathbuf = direntry.path();
                                if let Some(true) = visited_vertices.get(&node_pathbuf) {
                                    continue;
                                }
                                deque.push_back(node_pathbuf);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, str};
    #[test]
    fn test_passing_levenshtein() {
        let path = PathBuf::from("/home/user/foo/bar/baz.txt");
        let to_search = "baz";
        let ratio = close_enough(&path, to_search);
        assert!(ratio)
    }
    #[test]
    fn test_failing_levenshtein() {
        let path = PathBuf::from("/home/user/foo/bar/baz.txt");
        let to_search = "foo";
        let ratio = close_enough(&path, to_search);
        assert!(!ratio)
    }
    fn test_dget_utf8(to_search: &str, test_dir: PathBuf, expected: &PathBuf) {
        let mut fake_stdout: Vec<u8> = Vec::with_capacity(10);
        let run = {
            match dget(
                test_dir.clone(),
                to_search,
                Some(test_dir.as_path()),
                &mut fake_stdout,
            ) {
                Err(_) => false,
                Ok(_) => true,
            }
        };
        let stdout_print = match str::from_utf8(fake_stdout.as_ref()) {
            Err(e) => {
                eprintln!("{e}");
                return;
            }
            Ok(val) => val,
        };
        assert!(run);
        assert_eq!(PathBuf::from(stdout_print.trim()), *expected);
    }
    #[test]
    fn test_dget_utf8_cases_single() {
        let start = {
            match current_dir() {
                Err(_) => panic!("Test Failed! Cannot determine current directory\n"),
                Ok(val) => val,
            }
        };
        let test_dir = start.join("test_dir");
        test_dget_utf8(
            "apple",
            test_dir.clone(),
            &test_dir.join("bar").join("apple.config"),
        );
        test_dget_utf8("ham", test_dir.clone(), &test_dir.join("ham.txt"));
        test_dget_utf8("foo", test_dir.clone(), &test_dir.join("foo"));
        test_dget_utf8(
            "sandwich",
            test_dir.clone(),
            &test_dir.join("bar").join("sandwich.txt"),
        );
    }

    fn test_gitignore(test_dir: PathBuf, to_search: &str, gitignore_path: &str, expected: PathBuf) {
        let mut fake_stdout: Vec<u8> = Vec::new();
        let run = {
            match dget(
                test_dir.clone(),
                to_search,
                Some(test_dir.join(gitignore_path).as_path()),
                &mut fake_stdout,
            ) {
                Err(_) => false,
                Ok(_) => true,
            }
        };
        let stdout_print = match str::from_utf8(fake_stdout.as_ref()) {
            Err(e) => {
                println!("{e}");
                return;
            }
            Ok(val) => val,
        };
        assert!(run);
        assert_eq!(PathBuf::from(stdout_print.trim()), expected)
    }
    #[test]
    fn test_gitignore_cases() {
        let start = {
            match current_dir() {
                Err(_) => panic!("Test Failed! Cannot determine current directory\n"),
                Ok(val) => val,
            }
        };
        let test_dir = start.join("test_dir");
        test_gitignore(
            test_dir.clone(),
            "turkey",
            "./custom.ignore",
            PathBuf::new(),
        );
        test_gitignore(
            test_dir.clone(),
            "turkey",
            "",
            test_dir.join("chicken").join("turkey.file"),
        );
    }
}
