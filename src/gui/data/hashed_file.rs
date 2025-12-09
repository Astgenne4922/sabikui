use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use chrono::{DateTime, Utc};

use crate::{
    algorithms::{many_hash_one_file, many_hashes_many_files, one_hash_one_file},
    gui::data::table_columns::TableColumns,
};

pub type HashFunction = String;
pub type Digest = String;

#[derive(Default, Clone)]
pub struct HashedFile {
    pub path: PathBuf,
    digests: HashMap<HashFunction, Digest>,
}

impl HashedFile {
    pub fn new(path: &Path, algorithms: &[HashFunction]) -> Self {
        let mut new = Self {
            path: path.to_path_buf(),
            digests: HashMap::default(),
        };

        let hashes = many_hash_one_file(algorithms, path);

        new.digests = algorithms
            .iter()
            .map(std::string::ToString::to_string)
            .zip(hashes)
            .collect::<HashMap<_, _>>();

        new
    }

    pub fn build_vec(paths: &[PathBuf], algorithms: &[HashFunction]) -> Vec<Self> {
        let digests = many_hashes_many_files(algorithms, paths);

        paths
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let digests = &digests.iter().map(|alg| alg[i].clone()).collect::<Vec<_>>();
                Self {
                    path: file.clone(),
                    digests: algorithms
                        .iter()
                        .zip(digests)
                        .map(|(h, d)| (h.to_owned(), d.to_owned()))
                        .collect(),
                }
            })
            .collect()
    }

    pub fn file_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    pub fn last_edit(&self) -> String {
        let time: DateTime<Utc> = self.path.metadata().unwrap().modified().unwrap().into();
        time.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[cfg(unix)]
    pub fn size(&self) -> u64 {
        self.path.metadata().unwrap().size()
    }

    #[cfg(target_os = "windows")]
    pub fn size(&self) -> u64 {
        self.path.metadata().unwrap().file_size()
    }

    pub fn extension(&self) -> String {
        self.path.extension().unwrap().to_str().unwrap().to_owned()
    }

    pub fn get_digest(&self, algorithm: &HashFunction) -> Option<&Digest> {
        self.digests.get(algorithm)
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
            TableColumns::Path => self.path.display().to_string(),
            TableColumns::FileName => self.file_name(),
            TableColumns::Algorithms(alg) => self.get_digest(alg).unwrap().to_owned(),
            TableColumns::LastEdit => self.last_edit(),
            TableColumns::FileSize => self.size().to_string(),
            TableColumns::Extension => self.extension(),
        }
    }
}
