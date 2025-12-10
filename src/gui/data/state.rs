use std::collections::{HashMap, HashSet};

use crate::{
    algorithms,
    gui::data::{
        hashed_file::{HashFunction, HashedFile},
        table_columns::TableColumns,
    },
};

pub struct State {
    pub files: Vec<HashedFile>,
    pub columns: Vec<(TableColumns, bool)>,
    pub sorting_column: Option<(TableColumns, bool)>,
    pub algorithms: HashMap<HashFunction, bool>,
    pub always_on_top: bool,
    pub selected_rows: HashSet<usize>,
    pub last_selected: usize,
    pub to_copy: Option<String>,
}

impl State {
    pub fn new() -> Self {
        let hashes = algorithms::get_hash_functions();
        let mut cols = vec![(TableColumns::Path, true), (TableColumns::FileName, true)];
        cols.extend(hashes.iter().map(|h| (TableColumns::Algorithms(h.to_owned()), true)));
        cols.push((TableColumns::FileSize, true));
        cols.push((TableColumns::LastEdit, true));
        cols.push((TableColumns::Extension, true));

        Self {
            files: Vec::default(),
            columns: cols,
            sorting_column: None,
            algorithms: hashes.iter().map(|h| (h.to_owned(), true)).collect(),
            always_on_top: false,
            selected_rows: HashSet::default(),
            last_selected: Default::default(),
            to_copy: Option::default(),
        }
    }

    pub fn active_columns(&self) -> Vec<TableColumns> {
        self.columns
            .iter()
            .filter_map(|(col, is_checked)| is_checked.then_some(col.clone()))
            .collect::<Vec<_>>()
    }

    pub fn algorithm_list(&self) -> Vec<HashFunction> {
        let mut active_algorithms: Vec<String> = self
            .algorithms
            .iter()
            .filter_map(|(alg, is_checked)| is_checked.then_some(alg.to_owned()))
            .collect();
        active_algorithms.sort();
        active_algorithms
    }

    pub fn sort_table(&mut self, column: TableColumns) {
        let reverse = match &self.sorting_column {
            Some((col, is_reverse)) if *col == column => !is_reverse,
            _ => false,
        };

        self.files.sort_by_key(|f| f.get_from_column(&column));
        if reverse {
            self.files.reverse();
        }

        self.sorting_column = Some((column, reverse));
    }
}
