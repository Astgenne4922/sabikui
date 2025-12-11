use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::RangeInclusive,
    path::PathBuf,
    sync::mpsc,
};

use crate::gui::data::{
    hashed_file::{HashFunction, HashedFile},
    state::State,
    table_columns::TableColumns,
};

#[derive(Clone)]
pub enum Action {
    Refresh,
    Paste(String),
    SelectAll,
    DeselectAll,
    CopySelected,
    SaveSelected,
    ClearSelected,
    ClearAll,
    AddFiles(Option<Vec<PathBuf>>),
    AddFolders(Option<Vec<PathBuf>>),
    AddWildcard,
    CopyHash(HashFunction),
    SortBy(TableColumns),
    ToggleColumn(TableColumns),
    ToggleAlwaysOnTop,
    SelectSingleRow(usize),
    AddRowToSelection(usize),
    SelectRowRange(RangeInclusive<usize>),
}

pub struct ActionHandler {
    message_receiver: mpsc::Receiver<Action>,
}

impl ActionHandler {
    pub const fn new(message_receiver: mpsc::Receiver<Action>) -> Self {
        Self { message_receiver }
    }

    pub fn handle(&self, state: &mut State) {
        while let Ok(action) = self.message_receiver.try_recv() {
            match action {
                Action::Refresh => state.refresh(),
                Action::Paste(to_paste) => explorer_paste(&to_paste, state),
                Action::SelectAll => state.select_all(),
                Action::DeselectAll => state.deselect_all(),
                Action::CopySelected => {
                    state.to_copy = Some(copy_selected(
                        state.files(),
                        state.selected_rows(),
                        &state.active_columns(),
                    ));
                }
                Action::SaveSelected => save_selected(state.files(), state.selected_rows(), &state.active_columns()),
                Action::ClearSelected => state.clear_selected(),
                Action::ClearAll => state.clear_all(),
                Action::AddFiles(new_files) => add_files(new_files, state),
                Action::AddFolders(folders) => add_folders(folders, state),
                Action::AddWildcard => add_wildcard(),
                Action::CopyHash(alg) => state.to_copy = Some(copy_hash(&alg, state.files(), state.selected_rows())),
                Action::SortBy(column) => state.sort_table(column),
                Action::ToggleColumn(table_column) => state.toggle_column(&table_column),
                Action::ToggleAlwaysOnTop => state.toggle_always_on_top(),
                Action::SelectSingleRow(index) => state.select_single_row(index),
                Action::AddRowToSelection(index) => state.add_row_to_selection(index),
                Action::SelectRowRange(range) => state.select_range(range),
            }
        }
    }
}

fn explorer_paste(pasted: &str, state: &mut State) {
    let pasted_lines = pasted
        .lines()
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .collect::<Vec<_>>();
    state.add_files(&pasted_lines);
}

fn copy_selected(files: &[HashedFile], selected_rows: &HashSet<usize>, columns: &[TableColumns]) -> String {
    files
        .iter()
        .enumerate()
        .filter(|&(i, _)| selected_rows.contains(&i))
        .map(|(_, file)| {
            columns
                .iter()
                .map(|column| file.get_from_column(column))
                .collect::<Vec<_>>()
                .join("\t")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn save_selected(files: &[HashedFile], selected_rows: &HashSet<usize>, columns: &[TableColumns]) {
    if let Some(path) = &mut rfd::FileDialog::new().add_filter("csv", &["csv"]).save_file() {
        path.set_extension("csv");
        let to_save = columns
            .iter()
            .map(TableColumns::to_string)
            .collect::<Vec<_>>()
            .join(",");

        fs::write(
            path,
            format!(
                "{to_save}\n{}",
                copy_selected(files, selected_rows, columns).replace('\t', ",")
            ),
        )
        .unwrap();
    }
}

fn add_files(new_files: Option<Vec<PathBuf>>, state: &mut State) {
    let new_files = new_files.or_else(|| rfd::FileDialog::new().pick_files());

    if let Some(new_files) = new_files {
        let new_files = new_files.into_iter().filter(|f| f.is_file()).collect::<Vec<_>>();
        state.add_files(&new_files);
    }
}

fn add_folders(folders: Option<Vec<PathBuf>>, state: &mut State) {
    let folders = folders.or_else(|| rfd::FileDialog::new().pick_folders());

    if let Some(folders) = folders {
        state.add_files(&get_files(&folders));
    }
}

// TODO add wildcard - F4
fn add_wildcard() {
    println!("ADD WILDCARD");
}

fn copy_hash(alg: &HashFunction, files: &[HashedFile], selected_rows: &HashSet<usize>) -> String {
    files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| selected_rows.contains(&i).then_some(file.get_digest(alg)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path in paths {
        let mut dirs = VecDeque::new();

        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            dirs.push_back(path.clone());
        }

        while let Some(dir) = dirs.pop_front() {
            // TODO Handle errors
            for entry in dir.read_dir().unwrap().flatten() {
                let path = entry.path();

                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    dirs.push_back(path);
                }
            }
        }
    }

    files
}
