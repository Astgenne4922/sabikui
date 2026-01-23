use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::gui::{constants::labels, data::hashed_file::HashFunction};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TableColumns {
    Path,
    FileName,
    Algorithms(HashFunction),
    LastEdit,
    FileSize,
    Extension,
}

impl From<&str> for TableColumns {
    fn from(value: &str) -> Self {
        match value {
            "full path" => Self::Path,
            "filename" => Self::FileName,
            "file size (bytes)" => Self::FileSize,
            "edit time" => Self::LastEdit,
            "extension" => Self::Extension,
            _ => Self::Algorithms(value.to_string()),
        }
    }
}

impl Display for TableColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self {
            Self::Path => labels::FULL_PATH,
            Self::FileName => labels::FILENAME,
            Self::Algorithms(alg) => &alg.to_uppercase(),
            Self::LastEdit => labels::EDIT_TIME,
            Self::FileSize => labels::FILE_SIZE,
            Self::Extension => labels::EXTENSION,
        };
        write!(f, "{col}")
    }
}
