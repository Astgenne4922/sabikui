use std::{fs, path, process};

pub fn get_hash_functions() -> Vec<String> {
    fs::read_dir("./hash_functions")
        .unwrap()
        .map(|e| e.unwrap().path().file_stem().unwrap().to_str().unwrap().to_owned())
        .collect()
}

pub fn digest(function: &str, files: &[path::PathBuf]) -> Vec<String> {
    let hash = process::Command::new(format!("./hash_functions/{function}"))
        .args(files)
        .output()
        .unwrap();
    let binding = String::from_utf8(hash.stdout).unwrap();
    binding.lines().map(String::from).collect()
}

pub fn digest_many(functions: &[String], files: &[path::PathBuf]) -> Vec<Vec<String>> {
    functions.iter().map(|hash| digest(&hash, &files)).collect::<Vec<_>>()
}
