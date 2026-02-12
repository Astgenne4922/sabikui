use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::gui::data::{
    state::{AsyncAction, OpenWindow, State},
    table_columns::TableColumns,
};

#[derive(Clone)]
pub enum FileAction {
    Refresh,
    Paste(String),
    PickFiles,
    PickFolders,
    DroppedItems(Vec<PathBuf>),
    AddWildcard(String),
    ToggleColumn(TableColumns),
}

pub fn handle(state: Arc<Mutex<State>>, action: &FileAction) {
    match action {
        FileAction::Refresh => State::refresh(state),
        FileAction::Paste(to_paste) => {
            let pasted_lines: Vec<PathBuf> = to_paste
                .lines()
                .map(PathBuf::from)
                .filter(|path| path.exists())
                .collect();
            State::add_files(state, pasted_lines);
        }
        FileAction::PickFiles => pick_files(state),
        FileAction::PickFolders => pick_folders(state),
        FileAction::DroppedItems(folders) => State::add_files(state, get_files(folders)),
        FileAction::AddWildcard(wildcard) => {
            State::lock(&state).open_extra_window = OpenWindow::None;

            if let Ok(paths) = glob::glob(wildcard) {
                let list = paths.filter_map(Result::ok).collect::<Vec<_>>();
                State::add_files(state, get_files(&list));
            }
        }
        FileAction::ToggleColumn(table_column) => State::toggle_column(state, table_column),
    }
}

fn pick_files(state: Arc<Mutex<State>>) {
    State::lock(&state).async_action = AsyncAction::FileDialog;
    std::thread::spawn(move || {
        if let Some(files) = rfd::FileDialog::new().pick_files() {
            State::add_files(state, files.into_iter().filter(|f| f.is_file()).collect());
        }
    });
}

fn pick_folders(state: Arc<Mutex<State>>) {
    State::lock(&state).async_action = AsyncAction::FileDialog;
    std::thread::spawn(move || {
        if let Some(folders) = rfd::FileDialog::new().pick_folders() {
            State::add_files(state, get_files(&folders));
        }
    });
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
            for entry in dir.read_dir().expect("The directory should be readable").flatten() {
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
