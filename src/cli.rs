use std::collections::VecDeque;

use crate::algorithms;

pub mod parser;

#[expect(clippy::missing_panics_doc, reason = "Panic by design")]
pub fn run(args: parser::Inputs, algorithms: &[String]) {
    let files = match args {
        parser::Inputs::File { path } if path.exists() && path.is_file() => vec![path],
        parser::Inputs::Files { paths } if paths.iter().all(|p| p.exists() && p.is_file()) => paths,
        parser::Inputs::Directory { path } if path.exists() && path.is_dir() => {
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

            files
        }
        parser::Inputs::Wildcard { regex } => glob::glob(&regex)
            .expect("The glob pattern should be valid")
            .flatten()
            .filter(|entry| entry.is_file())
            .collect(),
        _ => panic!("Invalid argument. Run with --help to view usage."),
    };

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
}
