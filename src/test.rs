use crate::{close_enough, Args, DGET};
use clap;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::Parser;

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
    let test_dir_display = test_dir.display();
    let test_dir_as_string = test_dir_display.to_string();
    let test_dir_as_str = test_dir_as_string.as_str();
    let dummy_args = vec!["dget", "-s", test_dir_as_str, "-f", to_search];
    let args = Args::parse_from(dummy_args);
    //Make DGET accepts a dummy args somehow?
    for path in DGET::new(args).filter(|path| close_enough(path.as_path(), to_search)) {
        let disp = path.display();
        let as_string = disp.to_string();
        let as_str = as_string.as_str();
        writeln!(fake_stdout, "{as_str}").expect("BUG: writing to stdout should not fail");
    }
    let stdout_print = match str::from_utf8(fake_stdout.as_ref()) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(val) => val,
    };
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
    let test_dir_display = test_dir.display();
    let test_dir_as_string = test_dir_display.to_string();
    let test_dir_as_str = test_dir_as_string.as_str();
    let dummy_args = vec![
        "dget",
        "--start",
        test_dir_as_str,
        "--find",
        to_search,
        "--gitignore",
        gitignore_path,
    ];
    let args = Args::parse_from(dummy_args);
    for path in DGET::new(args).filter(|path| close_enough(path.as_path(), to_search)) {
        let disp = path.display();
        let as_string = disp.to_string();
        let as_str = as_string.as_str();
        writeln!(fake_stdout, "{as_str}").expect("BUG: writing to stdout should not fail");
    }
    let stdout_print = match str::from_utf8(fake_stdout.as_ref()) {
        Err(e) => {
            eprintln!("{e}");
            return;
        }
        Ok(val) => val,
    };
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
    let ignore_path = test_dir.join("custom.ignore");
    let ignore = ignore_path.to_str().unwrap_or("");
    test_gitignore(test_dir.clone(), "turkey", ignore, PathBuf::new());
    test_gitignore(
        test_dir.clone(),
        "turkey",
        "",
        test_dir.join("chicken").join("turkey.file"),
    );
}
