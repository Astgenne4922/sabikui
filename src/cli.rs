use crate::algorithms;

pub mod parser;

pub fn run(args: parser::Inputs) {
    let files = match args {
        parser::Inputs::File { path } if path.exists() && path.is_file() => vec![path],
        parser::Inputs::Files { paths } if paths.iter().all(|p| p.exists() && p.is_file()) => paths,
        parser::Inputs::Directory { path } if path.exists() && path.is_dir() => path // TODO recurse sub folders
            .read_dir()
            .unwrap()
            .filter_map(|entry| entry.as_ref().unwrap().path().is_file().then(|| entry.unwrap().path()))
            .collect(),
        parser::Inputs::Wildcard { regex } => glob::glob(&regex)
            .unwrap()
            .filter_map(|entry| entry.as_ref().unwrap().is_file().then(|| entry.unwrap()))
            .collect(),
        _ => panic!("Invalid argument"),
    };

    let functions = algorithms::get_hash_functions();

    let mut output = format!("filename,{}", functions.join(","));
    let digests = algorithms::digest_many(&functions, &files);

    for i in 0..files.len() {
        output = format!(
            "{output}\n{},{}",
            files[i].file_name().unwrap().to_str().unwrap(),
            digests.iter().map(|v| v[i].clone()).collect::<Vec<_>>().join(",")
        );
    }

    println!("{output}");
}
