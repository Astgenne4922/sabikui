//! Module containing the logic for the CLI mode of the app

use std::collections::VecDeque;

use crate::algorithms;

pub mod parser;

/// Runs the specified subcommand to get the files to process and hashes them with
/// the specified `algorithms`
///
/// # Errors
/// An error is returned if:
/// - `file`: the path doesn't exist or if it isn't a file
/// - `files`: at least one of the files doesn't exist or isn't a file
/// - `directory`: the directory doesn't exist or if it isn't a directory
/// - `wildcard`: the glob pattern is invalid
///
/// # Panics
/// A panic will occur when:
/// - `directory`: a directory can't be read
/// - a file name metadata can't be read
///
pub fn run(args: parser::Inputs, algorithms: &[String]) -> Result<(), &str> {
    let files = match args {
        parser::Inputs::File { path } => {
            if path.exists() && path.is_file() {
                Ok(vec![path])
            } else {
                Err("Invalid path. Run with --help to view usage.")
            }
        }
        parser::Inputs::Files { paths } => {
            if paths.iter().all(|p| p.exists() && p.is_file()) {
                Ok(paths)
            } else {
                Err("Invalid paths. Run with --help to view usage.")
            }
        }
        parser::Inputs::Directory { path } => {
            if path.exists() && path.is_dir() {
                let mut files = Vec::new();

                let mut dirs = VecDeque::new();

                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    dirs.push_back(path);
                }

                while let Some(dir) = dirs.pop_front() {
                    for entry in dir.read_dir().expect("The directory should be readable").flatten() {
                        let path = entry.path();

                        if path.is_file() {
                            files.push(path);
                        } else if path.is_dir() {
                            dirs.push_back(path);
                        }
                    }
                }

                Ok(files)
            } else {
                Err("Invalid directory. Run with --help to view usage.")
            }
        }
        parser::Inputs::Wildcard { regex } => glob::glob(&regex)
            .map_or(Err("Invalid glob pattern. Run with --help to view usage."), |glob| {
                Ok(glob.flatten().filter(|entry| entry.is_file()).collect())
            }),
    }?;

    let mut output = format!("filename,{}", algorithms.join(","));
    let digests = algorithms::calculate_hash(algorithms, &files);

    for i in 0..files.len() {
        output = format!(
            "{output}\n{},{}",
            files[i]
                .file_name()
                .expect("There should always be a file name")
                .to_string_lossy(),
            digests.iter().map(|v| v[i].clone()).collect::<Vec<_>>().join(",")
        );
    }

    println!("{output}");
    Ok(())
}
