use std::fmt::Display;

use crate::gui::data::hashed_file::{HashFunction, HashedFile};

#[derive(Clone)]
pub enum TableColumns {
    Path,
    FileName,
    Algorithms(HashFunction),
    LastEdit,
    FileSize,
    Extension,
}

impl Display for TableColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self {
            TableColumns::Path => "Full Path".to_owned(),
            TableColumns::FileName => "File Name".to_owned(),
            TableColumns::Algorithms(alg) => alg.to_ascii_uppercase(),
            TableColumns::LastEdit => "Last Edit".to_owned(),
            TableColumns::FileSize => "Size".to_owned(),
            TableColumns::Extension => "Extension".to_owned(),
        };
        write!(f, "{col}")
    }
}

impl TableColumns {
    pub fn from_file(&self, file: &HashedFile) -> String {
        match self {
            TableColumns::Path => file.path.display().to_string(),
            TableColumns::FileName => file.file_name(),
            TableColumns::Algorithms(alg) => file.get_digest(alg).unwrap().to_owned(),
            TableColumns::LastEdit => file.last_edit(),
            TableColumns::FileSize => file.size().to_string(),
            TableColumns::Extension => file.extension(),
        }
    }
}
