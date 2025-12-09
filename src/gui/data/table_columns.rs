use std::fmt::Display;

use crate::gui::data::hashed_file::HashFunction;

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
            Self::Path => "Full Path".to_owned(),
            Self::FileName => "File Name".to_owned(),
            Self::Algorithms(alg) => alg.to_ascii_uppercase(),
            Self::LastEdit => "Last Edit".to_owned(),
            Self::FileSize => "Size".to_owned(),
            Self::Extension => "Extension".to_owned(),
        };
        write!(f, "{col}")
    }
}
