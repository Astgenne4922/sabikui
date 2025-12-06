use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::mpsc,
};

use crate::gui::data::{
    hashed_file::{HashFunction, HashedFile},
    state::State,
    table_columns::TableColumns,
};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Action {
    Refresh,
    Paste,
    SelectAll,
    DeselectAll,
    CopySelected,
    SaveSelected,
    ClearSelected,
    ClearAll,
    AddFiles(Option<Vec<PathBuf>>),
    AddFolders(Option<Vec<PathBuf>>),
    AddWildcard,
    CopyHash(String),
    SortBy(String),
}

pub struct ActionHandler {
    message_receiver: mpsc::Receiver<Action>,
}

impl ActionHandler {
    pub fn new(message_receiver: mpsc::Receiver<Action>) -> Self {
        Self { message_receiver }
    }

    pub fn handle(&mut self, state: &mut State) {
        while let Ok(action) = self.message_receiver.try_recv() {
            match action {
                Action::Refresh => {
                    let alg_list = state.algorithm_list();
                    refresh(&mut state.files, &alg_list)
                }
                Action::Paste => explorer_paste(),
                Action::SelectAll => select_all(state.files.len(), &mut state.selected_rows, &mut state.last_selected),
                Action::DeselectAll => deselect_all(&mut state.selected_rows, &mut state.last_selected),
                Action::CopySelected => {
                    state.to_copy = Some(copy_selected(
                        &mut state.files,
                        &mut state.selected_rows,
                        &state
                            .columns
                            .iter()
                            .filter_map(|(col, is_checked)| if *is_checked { Some(col.clone()) } else { None })
                            .collect::<Vec<_>>(),
                    ))
                }
                Action::SaveSelected => save_selected(),
                Action::ClearSelected => {
                    clear_selected(&mut state.files, &mut state.selected_rows, &mut state.last_selected)
                }
                Action::ClearAll => clear_all(&mut state.files, &mut state.selected_rows, &mut state.last_selected),
                Action::AddFiles(new_files) => {
                    let alg_list = state.algorithm_list();
                    add_files(new_files, &mut state.files, &alg_list);
                }
                Action::AddFolders(folders) => {
                    let alg_list = state.algorithm_list();
                    add_folders(folders, &mut state.files, &alg_list);
                }
                Action::AddWildcard => add_wildcard(),
                Action::CopyHash(_) => copy_hash(),
                Action::SortBy(_) => sort_by(),
            }
        }
    }
}

fn refresh(files: &mut Vec<HashedFile>, algorithms: &[HashFunction]) {
    *files = HashedFile::build_vec(
        &get_files(&files.iter().map(|f| f.path.clone()).collect::<Vec<_>>()),
        algorithms,
    );
}

// TODO Explorer paste - CTRL+V
fn explorer_paste() {
    println!("EXPLORER PASTE");
}

fn select_all(num_rows: usize, selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    *selected_rows = HashSet::from_iter(0..num_rows);
    *last_selected = 0;
}

fn deselect_all(selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    selected_rows.clear();
    *last_selected = 0;
}

fn copy_selected(files: &mut Vec<HashedFile>, selected_rows: &mut HashSet<usize>, columns: &[TableColumns]) -> String {
    let mut output = String::default();

    for i in selected_rows.iter() {
        let file = &files[*i];

        let line = columns
            .iter()
            .map(|column| match column {
                TableColumns::Path => file.path.display().to_string(),
                TableColumns::FileName => file.file_name(),
                TableColumns::Algorithms(alg) => file.get_digest(alg).unwrap().to_owned(),
                TableColumns::LastEdit => file.last_edit(),
                TableColumns::FileSize => file.size().to_string(),
                TableColumns::Extension => file.extension(),
            })
            .collect::<Vec<_>>()
            .join("\t");

        output += &(line + "\n");
    }

    output
}

// TODO Save selected - CTRL+S
fn save_selected() {
    println!("SAVE SELECTED");
}

fn clear_selected(files: &mut Vec<HashedFile>, selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    *files = files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| (!selected_rows.contains(&i)).then(|| file.clone()))
        .collect::<Vec<_>>();
    selected_rows.clear();
    *last_selected = 0;
}

fn clear_all(files: &mut Vec<HashedFile>, selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    files.clear();
    selected_rows.clear();
    *last_selected = 0;
}

fn add_files(new_files: Option<Vec<PathBuf>>, files: &mut Vec<HashedFile>, algorithms: &[HashFunction]) {
    let new_files = new_files.or_else(|| rfd::FileDialog::new().pick_files());

    if let Some(new_files) = new_files {
        let new_files = new_files.into_iter().filter(|f| f.is_file()).collect::<Vec<_>>();
        files.extend(HashedFile::build_vec(&new_files, algorithms));
    }
}

fn add_folders(folders: Option<Vec<PathBuf>>, files: &mut Vec<HashedFile>, algorithms: &[HashFunction]) {
    let folders = folders.or_else(|| rfd::FileDialog::new().pick_folders());

    if let Some(folders) = folders {
        files.extend(HashedFile::build_vec(&get_files(&folders), algorithms));
    }
}

// TODO add wildcard - F4
fn add_wildcard() {
    println!("ADD WILDCARD");
}

// TODO Copy [ALG]
fn copy_hash() {
    println!("COPY HASH");
}

// TODO Sort By
fn sort_by() {
    println!("SORT BY");
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

        while !dirs.is_empty() {
            let dir = dirs.pop_front().unwrap();

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
