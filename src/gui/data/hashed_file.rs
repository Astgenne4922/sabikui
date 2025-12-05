use std::{
    collections::HashMap,
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
};

use crate::algorithms::{self, many_hash_one_file};

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

    pub fn file_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    pub fn last_edit(&self) -> std::time::SystemTime {
        self.path.metadata().unwrap().modified().unwrap()
    }

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
        self.digests.insert(
            algorithm.to_string(),
            algorithms::one_hash_one_file(algorithm, &self.path),
        );
    }

    pub fn remove_digest(&mut self, algorithm: &HashFunction) {
        self.digests.remove(algorithm);
    }
}
