use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use chrono::{DateTime, Local};

use crate::{
    algorithms::{many_hash_one_file, many_hashes_many_files, one_hash_one_file},
    gui::data::table_columns::TableColumns,
};

pub type HashFunction = String;
pub type Digest = String;

#[derive(Default, Clone)]
pub struct HashedFile {
    path: PathBuf,
    digests: HashMap<HashFunction, Digest>,
}

impl HashedFile {
    #[allow(unused)]
    pub fn new(path: &Path, algorithms: &[HashFunction]) -> Self {
        let mut new = Self {
            path: path.to_path_buf(),
            digests: HashMap::default(),
        };

        let hashes = many_hash_one_file(algorithms, path);

        new.digests = algorithms.iter().map(String::to_string).zip(hashes).collect();

        new
    }

    pub fn build_vec(paths: &[PathBuf], algorithms: &[HashFunction]) -> Vec<Self> {
        let digests = many_hashes_many_files(algorithms, paths);

        paths
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let digests = digests.iter().map(|alg| alg[i].clone());
                Self {
                    path: file.clone(),
                    digests: algorithms.iter().map(String::to_string).zip(digests).collect(),
                }
            })
            .collect()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .expect("If this method is called the file should be valid")
            .to_string_lossy()
            .to_string()
    }

    pub fn last_edit(&self) -> String {
        let time: DateTime<Local> = self
            .path
            .metadata()
            .expect("If this method is called the file should exists")
            .modified()
            .expect("Supported platforms should have this method")
            .into();
        time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[cfg(unix)]
    pub fn size(&self) -> u64 {
        self.path
            .metadata()
            .expect("If this method is called the file should exists")
            .size()
    }

    #[cfg(target_os = "windows")]
    pub fn size(&self) -> u64 {
        self.path
            .metadata()
            .expect("If this method is called the file should exists")
            .file_size()
    }

    pub fn extension(&self) -> String {
        self.path
            .extension()
            .map_or_else(|| String::from("None"), |e| e.to_string_lossy().to_string())
    }

    pub fn get_digest(&self, algorithm: &HashFunction) -> Digest {
        self.digests
            .get(algorithm)
            .expect("The searched hash function should always be present")
            .clone()
    }

    pub fn add_digest_for(&mut self, algorithm: &HashFunction) {
        self.digests
            .insert(algorithm.clone(), one_hash_one_file(algorithm, &self.path));
    }

    pub fn remove_digest(&mut self, algorithm: &HashFunction) {
        self.digests.remove(algorithm);
    }

    pub fn get_from_column(&self, column: &TableColumns) -> String {
        match column {
            TableColumns::Path => self.path.to_string_lossy().to_string(),
            TableColumns::FileName => self.file_name(),
            TableColumns::Algorithms(alg) => self.get_digest(alg),
            TableColumns::LastEdit => self.last_edit(),
            TableColumns::FileSize => self.size().to_string(),
            TableColumns::Extension => self.extension(),
        }
    }
}

impl PartialEq for HashedFile {
    fn eq(&self, other: &Self) -> bool {
        self.digests == other.digests
    }
}
