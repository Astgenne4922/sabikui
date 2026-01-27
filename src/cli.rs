use std::collections::VecDeque;

use crate::algorithms;

pub mod parser;

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
                // TODO Handle errors
                for entry in dir.read_dir().unwrap().flatten() {
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
            // TODO Handle errors
            .unwrap()
            .flatten()
            .filter(|entry| entry.is_file())
            .collect(),
        _ => panic!("Invalid argument"), // TODO Handle errors
    };

    let mut output = format!("filename,{}", algorithms.join(","));
    let digests = algorithms::many_hashes_many_files(algorithms, &files);

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
