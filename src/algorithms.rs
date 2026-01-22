use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn get_hash_functions() -> Vec<String> {
    let mut hashes: Vec<String> = fs::read_dir("./hash_functions")
        // TODO Handle errors
        .unwrap()
        .flatten()
        .map(|e| {
            e.path()
                .file_stem()
                .expect("There should always be a file name")
                .to_string_lossy()
                .to_string()
        })
        .collect();
    hashes.sort();
    hashes
}

pub fn one_hash_one_file(function: &str, file: &Path) -> String {
    let hash = Command::new(format!("./hash_functions/{function}"))
        .arg(file)
        .output()
        // TODO Handle errors
        .unwrap();
    String::from_utf8(hash.stdout).expect("Output from the executables should always be valid")
}

pub fn one_hash_many_files(function: &str, files: &[PathBuf]) -> Vec<String> {
    let mut output = Vec::new();

    for chunk in files.chunks(200) {
        let hash = Command::new(format!("./hash_functions/{function}"))
            .args(chunk)
            .output()
            // TODO Handle errors
            .unwrap();
        let binding = String::from_utf8(hash.stdout).expect("Output from the executables should always be valid");
        output.extend(binding.lines().map(String::from));
    }

    output
}

pub fn many_hash_one_file(functions: &[String], file: &Path) -> Vec<String> {
    functions.iter().map(|hash| one_hash_one_file(hash, file)).collect()
}

pub fn many_hashes_many_files(functions: &[String], files: &[PathBuf]) -> Vec<Vec<String>> {
    functions.iter().map(|hash| one_hash_many_files(hash, files)).collect()
}
