use std::fmt::{Debug, Display};

use rust_i18n::t;
use serde::{Deserialize, Serialize};

use crate::gui::data::hashed_file::HashFunction;

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
            "full_path" => Self::Path,
            "filename" => Self::FileName,
            "file_size" => Self::FileSize,
            "edit_time" => Self::LastEdit,
            "extension" => Self::Extension,
            _ => Self::Algorithms(value.to_string()),
        }
    }
}

impl Display for TableColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self {
            Self::Path => t!("Full Path").to_string(),
            Self::FileName => t!("Filename").to_string(),
            Self::Algorithms(alg) => alg.to_uppercase(),
            Self::LastEdit => t!("Edit Time").to_string(),
            Self::FileSize => t!("File Size").to_string(),
            Self::Extension => t!("Extension").to_string(),
        };
        write!(f, "{col}")
    }
}

impl Debug for TableColumns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self {
            Self::Path => "full_path",
            Self::FileName => "filename",
            Self::Algorithms(alg) => &alg.to_lowercase(),
            Self::LastEdit => "file_size",
            Self::FileSize => "edit_time",
            Self::Extension => "extension",
        };
        write!(f, "{col}")
    }
}
