use std::collections::{HashMap, HashSet};

use crate::{
    algorithms,
    gui::data::{hashed_file::HashedFile, table_columns::TableColumns},
};

pub struct State {
    pub files: Vec<HashedFile>,
    pub columns: Vec<(TableColumns, bool)>,
    pub algorithms: HashMap<String, bool>,
    pub always_on_top: bool,
    pub selected_rows: HashSet<usize>,
    pub last_selected: usize,
}

impl State {
    pub fn new() -> Self {
        let mut cols = vec![(TableColumns::Path, true), (TableColumns::FileName, true)];
        cols.extend(
            algorithms::get_hash_functions()
                .iter()
                .map(|h| (TableColumns::Algorithms(h.to_owned()), true)),
        );
        cols.push((TableColumns::FileSize, true));
        cols.push((TableColumns::LastEdit, true));
        cols.push((TableColumns::Extension, true));

        Self {
            files: Vec::default(),
            columns: cols,
            algorithms: algorithms::get_hash_functions()
                .iter()
                .map(|h| (h.to_owned(), true))
                .collect::<HashMap<_, _>>(),
            always_on_top: false,
            selected_rows: Default::default(),
            last_selected: Default::default(),
        }
    }

    pub fn algorithm_list(&self) -> Vec<String> {
        let mut active_algorithms = self
            .algorithms
            .iter()
            .filter_map(|(alg, is_checked)| if *is_checked { Some(alg.clone()) } else { None })
            .collect::<Vec<_>>();
        active_algorithms.sort();
        active_algorithms
    }
}
