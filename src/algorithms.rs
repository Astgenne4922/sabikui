//! The interface over the `hash_functions` folder
//!
//! The executables in the folder are executed using [`Command`]
use std::{io::Write, path::PathBuf, process::Command};

use crate::gui::constants;

/// Returns the `hash_functions` folder path
///
/// |Platform | Value                                                      |
/// | ------- | ---------------------------------------------------------- |
/// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config/hash_functions       |
/// | macOS   | `$HOME`/Library/Application Support/sabikui/hash_functions |
/// | Windows | `{FOLDERID_RoamingAppData}`\sabikui\hash_functions         |
fn get_directory() -> PathBuf {
    dirs::config_dir()
        .expect("There should be a config directory")
        .join(constants::CONFIG_DIRECTORY)
        .join(constants::ALG_DIRECTORY)
}

#[must_use]
#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
/// Returns a vector containing the names of the available executables under `hash_functions` (without extension if present)
pub fn get_hash_functions() -> Vec<String> {
    let mut hashes: Vec<String> = std::fs::read_dir(get_directory())
        .expect("The config directory should be readable")
        .flatten()
        .map(|e| format!("{}", e.path().display()))
        .collect();
    hashes.sort();
    hashes
}

#[must_use]
#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
/// Iterates over `functions` to spawn the corresponding executable with [`Command`] to calculate the hash of the list of `files`.
///
/// The files are passed to the executable as a temporary file named `input.txt`
/// The result is a vector of vectors of hashes of the files, the outer vector having the same order as the `functions` slice and the inner vectors
/// having  the same order as the `files` slice
pub fn calculate_hash(functions: &[String], files: &[PathBuf]) -> Vec<Vec<String>> {
    let temp_dir = tempfile::tempdir().expect("The temporary directory should have been created");
    let mut temp_file = std::fs::File::create(temp_dir.path().join("input.txt")).expect("The full path should exist");
    for file in files {
        writeln!(temp_file, "{}", file.display()).expect("The temporary file should be writable");
    }

    functions
        .iter()
        .map(|function| {
            let mut output = Vec::new();

            let hash = Command::new(get_directory().join(function))
                .arg("input.txt")
                .current_dir(&temp_dir)
                .output()
                .unwrap_or_else(|_| panic!("{function} failed to start"));
            assert!(
                hash.status.success(),
                "{function} command returned an error -> {}",
                String::from_utf8(hash.stderr).expect("Output from the executables should always be valid")
            );
            let binding = String::from_utf8(hash.stdout).expect("Output from the executables should always be valid");
            output.extend(binding.lines().map(String::from));

            output
        })
        .collect()
}
