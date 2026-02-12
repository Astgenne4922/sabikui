use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::gui::constants;

fn get_directory() -> PathBuf {
    dirs::config_dir()
        .expect("There should be a config directory")
        .join(constants::CONFIG_DIRECTORY)
        .join(constants::ALG_DIRECTORY)
}

#[must_use]
#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
pub fn get_hash_functions() -> Vec<String> {
    let mut hashes: Vec<String> = std::fs::read_dir(get_directory())
        .expect("The config directory should be readable")
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

#[must_use]
#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
pub fn one_hash_one_file(function: &str, file: &Path) -> String {
    let hash = Command::new(get_directory().join(function))
        .arg(file)
        .output()
        .expect("Something wrong with the hash executable");
    String::from_utf8(hash.stdout).expect("Output from the executables should always be valid")
}

#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
pub fn one_hash_many_files(function: &str, files: &[PathBuf]) -> Vec<String> {
    let mut output = Vec::new();

    for chunk in files.chunks(200) {
        let hash = Command::new(get_directory().join(function))
            .args(chunk)
            .output()
            .expect("Something wrong with the hash executable");
        let binding = String::from_utf8(hash.stdout).expect("Output from the executables should always be valid");
        output.extend(binding.lines().map(String::from));
    }

    output
}

#[must_use]
pub fn many_hash_one_file(functions: &[String], file: &Path) -> Vec<String> {
    functions.iter().map(|hash| one_hash_one_file(hash, file)).collect()
}

#[must_use]
pub fn many_hashes_many_files(functions: &[String], files: &[PathBuf]) -> Vec<Vec<String>> {
    functions.iter().map(|hash| one_hash_many_files(hash, files)).collect()
}
