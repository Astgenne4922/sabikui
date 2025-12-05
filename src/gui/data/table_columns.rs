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

impl ToString for TableColumns {
    fn to_string(&self) -> String {
        match self {
            TableColumns::Path => "Full Path".to_owned(),
            TableColumns::FileName => "File Name".to_owned(),
            TableColumns::Algorithms(alg) => alg.to_ascii_uppercase(),
            TableColumns::LastEdit => "Last Edit".to_owned(),
            TableColumns::FileSize => "Size".to_owned(),
            TableColumns::Extension => "Extension".to_owned(),
        }
    }
}
