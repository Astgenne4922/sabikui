use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::mpsc,
};

use crate::gui::data::{
    hashed_file::{HashFunction, HashedFile},
    state::State,
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
    AddFolder,
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
                    let algs = &state.algorithm_list();
                    refresh(&mut state.files, algs);
                }
                Action::Paste => explorer_paste(),
                Action::SelectAll => select_all(state.files.len(), &mut state.selected_rows, &mut state.last_selected),
                Action::DeselectAll => deselect_all(&mut state.selected_rows, &mut state.last_selected),
                Action::CopySelected => copy_selected(),
                Action::SaveSelected => save_selected(),
                Action::ClearSelected => {
                    clear_selected(&mut state.files, &mut state.selected_rows, &mut state.last_selected)
                }
                Action::ClearAll => clear_all(&mut state.files, &mut state.selected_rows, &mut state.last_selected),
                Action::AddFiles(new_files) => {
                    let alg_list = state.algorithm_list();
                    add_files(new_files, &mut state.files, &alg_list);
                }
                Action::AddWildcard => add_wildcard(),
                Action::CopyHash(_) => copy_hash(),
                Action::SortBy(_) => sort_by(),
            }
        }
    }
}

// TODO Refresh - F5
fn refresh(files: &mut [HashedFile], algorithms: &[HashFunction]) {
    for file in files {
        *file = HashedFile::new(&file.path, algorithms);
    }
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

// TODO Copy Selected - CTRL+C
fn copy_selected() {
    println!("COPY SELECTED");
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
