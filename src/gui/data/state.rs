use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    path::PathBuf,
};

use crate::{
    algorithms,
    gui::data::{
        hashed_file::{HashFunction, HashedFile},
        table_columns::TableColumns,
    },
};

pub struct State {
    files: Vec<HashedFile>,
    columns: Vec<(TableColumns, bool)>,
    sorting_column: Option<(TableColumns, bool)>,
    algorithms: HashMap<HashFunction, bool>,
    always_on_top: bool,
    mark_same: bool,
    selected_rows: HashSet<usize>,
    last_selected: usize,
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
            mark_same: true,
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

    pub fn refresh(&mut self) {
        self.files = HashedFile::build_vec(
            &self.files.iter().map(HashedFile::get_path).collect::<Vec<_>>(),
            &self.algorithm_list(),
        );
    }

    pub fn select_all(&mut self) {
        self.selected_rows = (0..self.files.len()).collect();
        self.last_selected = 0;
    }

    pub fn select_single_row(&mut self, index: usize) {
        let a = self.selected_rows.contains(&index);
        self.deselect_all();
        if a {
            self.selected_rows.insert(index);
            self.last_selected = index;
        } else {
            self.last_selected = 0;
        }
    }

    pub fn add_row_to_selection(&mut self, index: usize) {
        if self.selected_rows.contains(&index) {
            self.selected_rows.remove(&index);
        } else {
            self.selected_rows.insert(index);
        }
        self.last_selected = index;
    }

    pub fn select_range(&mut self, range: RangeInclusive<usize>) {
        self.deselect_all();
        for i in range {
            self.selected_rows.insert(i);
        }
    }

    pub fn deselect_all(&mut self) {
        self.selected_rows.clear();
        self.last_selected = 0;
    }

    pub fn clear_selected(&mut self) {
        self.files = self
            .files
            .iter()
            .enumerate()
            .filter_map(|(i, file)| (!self.selected_rows.contains(&i)).then_some(file.clone()))
            .collect();
        self.deselect_all();
    }

    pub fn clear_all(&mut self) {
        self.files.clear();
        self.deselect_all();
    }

    pub fn add_files(&mut self, files: &[PathBuf]) {
        self.files.extend(HashedFile::build_vec(files, &self.algorithm_list()));
    }

    pub fn toggle_column(&mut self, column: &TableColumns) {
        let (column, is_checked) = self
            .columns
            .iter_mut()
            .find(|(c, _)| c == column)
            .expect("The column should always be present");

        *is_checked = !*is_checked;
        if let TableColumns::Algorithms(alg) = column {
            for file in &mut self.files {
                if *is_checked {
                    file.add_digest_for(alg);
                } else {
                    file.remove_digest(alg);
                }
            }
        }
    }

    pub const fn toggle_always_on_top(&mut self) {
        self.always_on_top = !self.always_on_top;
    }

    pub const fn toggle_mark_same(&mut self) {
        self.mark_same = !self.mark_same;
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

    pub fn pair_same_hashes(&self) -> Vec<Vec<usize>> {
        let mut pairs = Vec::new();

        for (idx1, file1) in self.files.iter().enumerate() {
            let mut pair = vec![idx1];

            for (idx2, file2) in self.files.iter().enumerate() {
                if file1 == file2 {
                    pair.push(idx2);
                }
            }

            if pair.len() > 1 {
                pairs.push(pair);
            }
        }

        pairs
    }
}

// GETTERS
impl State {
    pub const fn files(&self) -> &Vec<HashedFile> {
        &self.files
    }

    pub const fn columns(&self) -> &Vec<(TableColumns, bool)> {
        &self.columns
    }

    pub const fn sorting_column(&self) -> Option<&(TableColumns, bool)> {
        self.sorting_column.as_ref()
    }

    pub const fn always_on_top(&self) -> &bool {
        &self.always_on_top
    }

    pub const fn mark_same(&self) -> &bool {
        &self.mark_same
    }

    pub const fn selected_rows(&self) -> &HashSet<usize> {
        &self.selected_rows
    }

    pub const fn last_selected(&self) -> &usize {
        &self.last_selected
    }
}
