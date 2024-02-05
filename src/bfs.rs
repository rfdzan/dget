use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn bfs_main() {
    let start = PathBuf::from("/home/user/Documents");
    match bfs(start) {
        Err(e) => println!("{e}"),
        Ok(_) => (),
    }
}
fn bfs(start: PathBuf) -> io::Result<()> {
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
            writeln!(stdout, "{path:?}")?;
            if path.is_file() || path.is_symlink() {
                visited_vertices.insert(path.clone(), true);
                deque.push_back(path);
                continue;
            }
            visited_vertices.insert(path.clone(), true);
            match std::fs::read_dir(&path) {
                Err(e) => {
                    writeln!(stdout, "{e} {path:?}")?;
                    deque.push_back(path);
                    continue;
                }
                Ok(nodes) => {
                    for node in nodes {
                        match node {
                            Err(e) => {
                                writeln!(stdout, "{e}, {path:?}")?;
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
