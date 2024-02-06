use levenshtein::levenshtein;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn dget_main(starting_path: &str, to_search: &str) {
    let start = PathBuf::from(starting_path);
    match dget(start, to_search) {
        Err(e) => println!("{e}"),
        Ok(_) => (),
    }
}
fn close_enough(path: &PathBuf, to_search: &str) -> bool {
    match path.file_stem().unwrap_or_default().to_str() {
        None => false,
        Some(path_name) => {
            let edit_distance = match i32::try_from(levenshtein(path_name, to_search)) {
                Err(_) => return false,
                Ok(val) => val,
            };
            let arr =  [path_name.chars().count(), to_search.chars().count()];
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
                        return true
                    }
                    false
                }
            } 
        }
    }
}
fn dget(start: PathBuf, to_search: &str) -> io::Result<()> {
    let mut visited_vertices = HashMap::with_capacity(1000);
    let mut deque = VecDeque::with_capacity(1000);
    visited_vertices.insert(start.clone(), false);
    deque.push_back(start);

    let mut stdout = io::stdout().lock();
    while !deque.is_empty() {
        let current_node = deque.pop_front();
        if let Some(path) = current_node {
            if let Some(true) = visited_vertices.get(&path) {
                continue;
            }
            if close_enough(&path, to_search) {
                let disp = path.display();
                writeln!(stdout, "{disp}")?;
            }
            if path.is_file() || path.is_symlink() {
                visited_vertices.insert(path.clone(), true);
                deque.push_back(path);
                continue;
            }
            visited_vertices.insert(path.clone(), true);
            match std::fs::read_dir(&path) {
                Err(_) => {
                    // writeln!(stdout, "{e} {path:?}")?;
                    deque.push_back(path);
                    continue;
                }
                Ok(nodes) => {
                    for node in nodes {
                        match node {
                            Err(_) => {
                                // writeln!(stdout, "{e}, {path:?}")?;
                                deque.push_back(path.clone());
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
