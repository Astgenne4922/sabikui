use std::fmt::Display;

use crate::gui::data::{constants::labels, hashed_file::HashFunction};

#[derive(Clone, PartialEq, Eq, Hash)]
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
            Self::Path => labels::FULL_PATH,
            Self::FileName => labels::FILENAME,
            Self::Algorithms(alg) => &alg.to_ascii_uppercase(),
            Self::LastEdit => labels::EDIT_TIME,
            Self::FileSize => labels::FILE_SIZE,
            Self::Extension => labels::EXTENSION,
        };
        write!(f, "{col}")
    }
}
