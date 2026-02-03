use std::{
    ops::RangeInclusive,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use egui::{Pos2, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    algorithms::{self, one_hash_many_files},
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
    AddWildcard,
}

#[derive(PartialEq, Eq)]
pub enum AsyncAction {
    None,
    FileDialog,
    HashProcessing,
}

pub struct State {
    files: Vec<HashedFile>,
    columns: Vec<(TableColumns, bool)>,
    sorting_column: Option<(TableColumns, bool)>,
    always_on_top: bool,
    mark_same: bool,
    same_hash_index: Vec<Digest>,
    selected_rows: Vec<usize>,
    last_selected: usize,
    pub to_copy: Option<String>,
    pub drag_start: Option<(Pos2, Vec2)>,
    pub drag_start_index: Option<usize>,
    pub open_extra_window: OpenWindow,
    pub async_action: AsyncAction,
}

impl Default for State {
    fn default() -> Self {
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
            open_extra_window: OpenWindow::None,
            async_action: AsyncAction::None,
        }
    }
}

impl State {
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

    pub fn refresh(this: Arc<Mutex<Self>>) {
        Self::hash_processing(this, move |this| {
            let (alg_list, files) = {
                let lock = this.lock().expect("The lock was poisoned");

                (
                    lock.algorithm_list(),
                    lock.files.iter().map(HashedFile::get_path).collect::<Vec<_>>(),
                )
            };

            let refreshed = HashedFile::build_vec(&files, &alg_list);

            let mut lock = this.lock().expect("The lock was poisoned");
            lock.files.extend(refreshed);
            let sorting_column = &lock.sorting_column;
            if let Some((column, reverse)) = sorting_column.clone() {
                lock.files.sort_by_key(|f| f.get_from_column(&column));
                if reverse {
                    lock.files.reverse();
                }
            }

            lock.pair_same_hashes();
        });
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

    pub fn add_files(this: Arc<Mutex<Self>>, files: Vec<PathBuf>) {
        let mapped_files: Vec<_> = this
            .lock()
            .expect("The lock was poisoned")
            .files
            .iter()
            .map(HashedFile::get_path)
            .collect();

        Self::hash_processing(this, move |this| {
            let alg_list = this.lock().expect("The lock was poisoned").algorithm_list();

            let new_files = HashedFile::build_vec(
                &files
                    .iter()
                    .filter(|&f| !mapped_files.contains(f))
                    .cloned()
                    .collect::<Vec<_>>(),
                &alg_list,
            );

            let mut lock = this.lock().expect("The lock was poisoned");
            lock.files.extend(new_files);
            let sorting_column = &lock.sorting_column;
            if let Some((column, reverse)) = sorting_column.clone() {
                lock.files.sort_by_key(|f| f.get_from_column(&column));
                if reverse {
                    lock.files.reverse();
                }
            }

            lock.pair_same_hashes();
        });
    }

    pub fn update_column_order(&mut self, from: usize, to: usize) {
        egui_dnd::utils::shift_vec(from, to, &mut self.columns);
    }

    pub fn toggle_column(this: Arc<Mutex<Self>>, column: &TableColumns) {
        let lock = this.lock().expect("The lock was poisoned");
        let (column, is_checked) = {
            let (column, is_checked) = lock
                .columns
                .iter()
                .find(|(c, _)| c == column)
                .expect("The column should always be present");

            (column.clone(), !*is_checked)
        };
        drop(lock);

        if let TableColumns::Algorithms(alg) = column.clone() {
            if is_checked {
                Self::hash_processing(this, move |this| {
                    let files: Vec<_> = this
                        .lock()
                        .expect("The lock was poisoned")
                        .files
                        .iter()
                        .map(HashedFile::get_path)
                        .collect();
                    let new_column = one_hash_many_files(&alg, &files);

                    let mut lock = this.lock().expect("The lock was poisoned");
                    for (i, hash) in new_column.iter().enumerate() {
                        lock.files[i].add_digest_for(&alg, hash);
                    }

                    let (_, is_checked) = lock
                        .columns
                        .iter_mut()
                        .find(|(c, _)| *c == column)
                        .expect("The column should always be present");
                    *is_checked = !*is_checked;

                    lock.pair_same_hashes();
                    drop(lock);
                });
            } else {
                let mut lock = this.lock().expect("The lock was poisoned");
                for file in &mut lock.files {
                    file.remove_digest(&alg);
                }

                let (_, is_checked) = lock
                    .columns
                    .iter_mut()
                    .find(|(c, _)| *c == column)
                    .expect("The column should always be present");
                *is_checked = !*is_checked;

                lock.pair_same_hashes();
                drop(lock);
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

        if self.algorithm_list().is_empty() {
            return;
        }

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

    fn hash_processing<F>(this: Arc<Mutex<Self>>, to_process: F)
    where
        F: FnOnce(Arc<Mutex<Self>>) + Send + 'static,
    {
        std::thread::spawn(move || {
            this.lock().expect("The lock was poisoned").async_action = AsyncAction::HashProcessing;

            to_process(Arc::clone(&this));

            this.lock().expect("The lock was poisoned").async_action = AsyncAction::None;
        });
    }

    pub fn lock(this: &Arc<Mutex<Self>>) -> MutexGuard<'_, Self> {
        this.lock().expect("The lock was poisoned")
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

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pers = StatePersistence::deserialize(deserializer)?;
        let hashes = algorithms::get_hash_functions();

        Ok(Self {
            files: Vec::default(),
            columns: pers
                .columns
                .iter()
                .map(|(col, check)| (TableColumns::from(col.as_ref()), check.as_bool().unwrap()))
                .filter(|col| match col {
                    (TableColumns::Algorithms(alg), _) => hashes.contains(alg),
                    _ => true,
                })
                .collect(),
            sorting_column: None,
            always_on_top: pers.options.always_on_top,
            mark_same: pers.options.mark_same,
            same_hash_index: Vec::default(),
            selected_rows: Vec::default(),
            last_selected: Default::default(),
            to_copy: Option::default(),
            drag_start: None,
            drag_start_index: None,
            open_extra_window: OpenWindow::None,
            async_action: AsyncAction::None,
        })
    }
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        StatePersistence::new(&self.columns, self.always_on_top, self.mark_same).serialize(serializer)
    }
}

#[derive(Serialize, Deserialize)]
struct StatePersistence {
    options: StateOptions,
    columns: toml::Table,
}

impl StatePersistence {
    fn new(columns: &[(TableColumns, bool)], always_on_top: bool, mark_same: bool) -> Self {
        Self {
            options: StateOptions {
                always_on_top,
                mark_same,
            },
            columns: columns
                .iter()
                .map(|(col, check)| (col.to_string().to_lowercase(), toml::Value::Boolean(*check)))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StateOptions {
    always_on_top: bool,
    mark_same: bool,
}
