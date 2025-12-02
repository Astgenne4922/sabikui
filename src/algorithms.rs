use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn get_hash_functions() -> Vec<String> {
    fs::read_dir("./hash_functions")
        .unwrap()
        .map(|e| e.unwrap().path().file_stem().unwrap().to_str().unwrap().to_owned())
        .collect()
}

pub fn one_hash_one_file(function: &str, file: &Path) -> String {
    let hash = Command::new(format!("./hash_functions/{function}"))
        .arg(file)
        .output()
        .unwrap();
    String::from_utf8(hash.stdout).unwrap()
}

pub fn one_hash_many_files(function: &str, files: &[PathBuf]) -> Vec<String> {
    let hash = Command::new(format!("./hash_functions/{function}"))
        .args(files)
        .output()
        .unwrap();
    let binding = String::from_utf8(hash.stdout).unwrap();
    binding.lines().map(String::from).collect()
}

pub fn many_hash_one_file(functions: &[String], file: &Path) -> Vec<String> {
    functions.iter().map(|hash| one_hash_one_file(hash, file)).collect()
}

pub fn many_hashes_many_files(functions: &[String], files: &[PathBuf]) -> Vec<Vec<String>> {
    functions
        .iter()
        .map(|hash| one_hash_many_files(hash, files))
        .collect::<Vec<_>>()
}
