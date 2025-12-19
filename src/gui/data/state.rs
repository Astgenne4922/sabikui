use std::{ops::RangeInclusive, path::PathBuf, sync::Arc};

use egui::{Pos2, Vec2, mutex::RwLock};

use crate::{
    algorithms,
    gui::data::{
        hashed_file::{Digest, HashFunction, HashedFile},
        table_columns::TableColumns,
    },
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum OpenWindow {
    None,
    About,
    Keybinds,
    ChooseColunms,
}

pub struct State {
    files: Vec<HashedFile>,
    pub columns: Vec<(TableColumns, bool)>,
    sorting_column: Option<(TableColumns, bool)>,
    always_on_top: bool,
    mark_same: bool,
    same_hash_index: Vec<Digest>,
    selected_rows: Vec<usize>,
    last_selected: usize,
    pub to_copy: Option<String>,
    pub drag_start: Option<(Pos2, Vec2)>,
    pub drag_start_index: Option<usize>,
    pub open_extra_window: Arc<RwLock<OpenWindow>>,
}

impl State {
    pub fn new() -> Self {
        let hashes = algorithms::get_hash_functions();
        let mut cols = vec![(TableColumns::Path, true), (TableColumns::FileName, true)];
        cols.extend(hashes.iter().map(|h| (TableColumns::Algorithms(h.to_owned()), false)));
        cols.push((TableColumns::FileSize, true));
        cols.push((TableColumns::LastEdit, true));
        cols.push((TableColumns::Extension, true));

        Self {
            files: Vec::default(),
            columns: cols,
            sorting_column: None,
            always_on_top: false,
            mark_same: true,
            same_hash_index: Vec::default(),
            selected_rows: Vec::default(),
            last_selected: Default::default(),
            to_copy: Option::default(),
            drag_start: None,
            drag_start_index: None,
            open_extra_window: Arc::new(RwLock::new(OpenWindow::None)),
        }
    }

    pub fn active_columns(&self) -> Vec<TableColumns> {
        self.columns
            .iter()
            .filter_map(|(col, is_checked)| is_checked.then_some(col.clone()))
            .collect()
    }

    pub fn algorithm_list(&self) -> Vec<HashFunction> {
        let mut active_algorithms: Vec<String> = self
            .columns
            .iter()
            .filter_map(|(col, is_checked)| {
                if let TableColumns::Algorithms(alg) = col {
                    is_checked.then_some(alg.clone())
                } else {
                    None
                }
            })
            .collect();
        active_algorithms.sort();
        active_algorithms
    }

    pub fn refresh(&mut self) {
        self.files = HashedFile::build_vec(
            &self.files.iter().map(HashedFile::get_path).collect::<Vec<_>>(),
            &self.algorithm_list(),
        );

        if let Some((column, reverse)) = &self.sorting_column {
            self.files.sort_by_key(|f| f.get_from_column(column));
            if *reverse {
                self.files.reverse();
            }
        }

        self.pair_same_hashes();
    }

    pub fn select_all(&mut self) {
        self.selected_rows = (0..self.files.len()).collect();
        self.last_selected = 0;
    }

    pub fn select_single_row(&mut self, index: usize) {
        let a = self.selected_rows.contains(&index);
        self.deselect_all();
        if !a {
            self.selected_rows.push(index);
            self.last_selected = index;
        }
    }

    pub fn add_row_to_selection(&mut self, index: usize) {
        if self.selected_rows.contains(&index) {
            self.selected_rows.pop_if(|r| *r == index);
        } else {
            self.selected_rows.push(index);
        }
        self.last_selected = index;
    }

    pub fn select_range(&mut self, range: RangeInclusive<usize>) {
        self.selected_rows.clear();
        self.selected_rows.extend(range);
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
        self.pair_same_hashes();
    }

    pub fn clear_all(&mut self) {
        self.files.clear();
        self.deselect_all();
        self.same_hash_index.clear();
    }

    pub fn add_files(&mut self, files: &[PathBuf]) {
        self.files.extend(HashedFile::build_vec(files, &self.algorithm_list()));

        if let Some((column, reverse)) = &self.sorting_column {
            self.files.sort_by_key(|f| f.get_from_column(column));
            if *reverse {
                self.files.reverse();
            }
        }

        self.pair_same_hashes();
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

    fn pair_same_hashes(&mut self) {
        self.same_hash_index = Vec::new();

        let alg = &self.algorithm_list()[0];

        for file1 in &self.files {
            let mut count = 0;

            if self.same_hash_index.contains(&file1.get_digest(alg)) {
                continue;
            }

            for file2 in &self.files {
                if file1 == file2 {
                    count += 1;
                }
            }

            if count > 1 {
                self.same_hash_index.push(file1.get_digest(alg));
            }
        }
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

    pub const fn same_hash_index(&self) -> &Vec<Digest> {
        &self.same_hash_index
    }

    pub const fn selected_rows(&self) -> &Vec<usize> {
        &self.selected_rows
    }

    pub const fn last_selected(&self) -> &usize {
        &self.last_selected
    }
}
