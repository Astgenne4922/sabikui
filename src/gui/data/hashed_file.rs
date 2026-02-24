use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use chrono::{DateTime, Local};

use crate::{algorithms, gui::data::table_columns::TableColumns};

pub type HashFunction = String;
pub type Digest = String;

#[derive(Default, Clone)]
pub struct HashedFile {
    path: PathBuf,
    digests: HashMap<HashFunction, Digest>,
    file_name: String,
    last_edit: String,
    size: u64,
    extension: String,
}

impl HashedFile {
    fn new(path: &Path, digests: HashMap<HashFunction, Digest>) -> Self {
        Self {
            path: path.to_path_buf(),
            digests,
            file_name: path
                .file_name()
                .expect("If this method is called the file should be valid")
                .to_string_lossy()
                .into(),
            last_edit: {
                let time: DateTime<Local> = path
                    .metadata()
                    .expect("If this method is called the file should exists")
                    .modified()
                    .expect("Supported platforms should have this method")
                    .into();
                time.format("%Y-%m-%d %H:%M:%S").to_string()
            },
            #[cfg(target_os = "windows")]
            size: path
                .metadata()
                .expect("If this method is called the file should exists")
                .file_size(),
            #[cfg(unix)]
            size: path
                .metadata()
                .expect("If this method is called the file should exists")
                .size(),
            extension: path
                .extension()
                .map_or_else(|| String::from("None"), |e| e.to_string_lossy().to_string()),
        }
    }

    pub fn build_vec(paths: &[PathBuf], algorithms: &[HashFunction]) -> Vec<Self> {
        let digests = algorithms::calculate_hash(algorithms, paths);

        paths
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let digests = digests.iter().map(|alg| alg[i].clone());
                Self::new(file, algorithms.iter().map(String::to_string).zip(digests).collect())
            })
            .collect()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_digest(&self, algorithm: &HashFunction) -> Digest {
        self.digests
            .get(algorithm)
            .expect("The searched hash function should always be present")
            .clone()
    }

    pub fn add_digest_for(&mut self, algorithm: &HashFunction, hash: &str) {
        self.digests.insert(algorithm.clone(), hash.to_string());
    }

    pub fn remove_digest(&mut self, algorithm: &HashFunction) {
        self.digests.remove(algorithm);
    }

    pub fn get_from_column(&self, column: &TableColumns) -> String {
        match column {
            TableColumns::Path => self.path.display().to_string(),
            TableColumns::FileName => self.file_name.clone(),
            TableColumns::Algorithms(alg) => self.get_digest(alg),
            TableColumns::LastEdit => self.last_edit.clone(),
            TableColumns::FileSize => self.size.to_string(),
            TableColumns::Extension => self.extension.clone(),
        }
    }
}

impl PartialEq for HashedFile {
    fn eq(&self, other: &Self) -> bool {
        self.digests == other.digests
    }
}
