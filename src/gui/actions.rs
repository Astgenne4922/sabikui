use std::{
    collections::{HashSet, VecDeque},
    fs,
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
            let alg_list = state.algorithm_list();

            match action {
                Action::Refresh => {
                    refresh(&mut state.files, &alg_list);
                }
                Action::Paste(to_paste) => {
                    explorer_paste(&to_paste, &mut state.files, &alg_list);
                }
                Action::SelectAll => select_all(state.files.len(), &mut state.selected_rows, &mut state.last_selected),
                Action::DeselectAll => deselect_all(&mut state.selected_rows, &mut state.last_selected),
                Action::CopySelected => {
                    state.to_copy = Some(copy_selected(
                        &state.files,
                        &state.selected_rows,
                        &state.active_columns(),
                    ));
                }
                Action::SaveSelected => save_selected(&state.files, &state.selected_rows, &state.active_columns()),
                Action::ClearSelected => {
                    clear_selected(&mut state.files, &mut state.selected_rows, &mut state.last_selected);
                }
                Action::ClearAll => clear_all(&mut state.files, &mut state.selected_rows, &mut state.last_selected),
                Action::AddFiles(new_files) => {
                    add_files(new_files, &mut state.files, &alg_list);
                }
                Action::AddFolders(folders) => {
                    add_folders(folders, &mut state.files, &alg_list);
                }
                Action::AddWildcard => add_wildcard(),
                Action::CopyHash(alg) => {
                    state.to_copy = Some(copy_hash(&alg, &state.files, &state.selected_rows));
                }
                Action::SortBy(_) => sort_by(),
            }
        }
    }
}

fn refresh(files: &mut Vec<HashedFile>, algorithms: &[HashFunction]) {
    *files = HashedFile::build_vec(
        &get_files(&files.iter().map(HashedFile::get_path).collect::<Vec<_>>()),
        algorithms,
    );
}

fn explorer_paste(pasted: &str, files: &mut Vec<HashedFile>, algorithms: &[HashFunction]) {
    let pasted_lines = pasted.lines().map(PathBuf::from).filter(|path| path.exists()).collect();
    add_folders(Some(pasted_lines), files, algorithms);
}

fn select_all(num_rows: usize, selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    *selected_rows = (0..num_rows).collect();
    *last_selected = 0;
}

fn deselect_all(selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    selected_rows.clear();
    *last_selected = 0;
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

fn clear_selected(files: &mut Vec<HashedFile>, selected_rows: &mut HashSet<usize>, last_selected: &mut usize) {
    *files = files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| (!selected_rows.contains(&i)).then_some(file.clone()))
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

fn copy_hash(alg: &HashFunction, files: &[HashedFile], selected_rows: &HashSet<usize>) -> String {
    files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| selected_rows.contains(&i).then_some(file.get_digest(alg)))
        .collect::<Vec<_>>()
        .join("\n")
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
