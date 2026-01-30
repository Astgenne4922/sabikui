use std::{collections::VecDeque, path::PathBuf, sync::mpsc::Sender};

use crate::gui::{
    actions::Action,
    data::{
        state::{AsyncAction, OpenWindow, State},
        table_columns::TableColumns,
    },
};

#[derive(Clone)]
pub enum FileAction {
    Refresh,
    Paste(String),
    PickFiles,
    PickFolders,
    AddFiles(Vec<PathBuf>),
    AddFolders(Vec<PathBuf>),
    AddWildcard(String),
    ToggleColumn(TableColumns),
}

pub fn handle(state: &mut State, action: &FileAction, sender: &Sender<Action>) {
    match action {
        FileAction::Refresh => state.refresh(),
        FileAction::Paste(to_paste) => explorer_paste(to_paste, state),
        FileAction::PickFiles => pick_files(state, sender.clone()),
        FileAction::PickFolders => pick_folders(state, sender.clone()),
        FileAction::AddFiles(new_files) => {
            state.async_action = AsyncAction::None;
            state.add_files(new_files);
        }
        FileAction::AddFolders(folders) => {
            state.async_action = AsyncAction::None;
            state.add_files(&get_files(folders));
        }
        FileAction::AddWildcard(wildcard) => {
            state.open_extra_window = OpenWindow::None;
            add_wildcard(wildcard, state);
        }
        FileAction::ToggleColumn(table_column) => state.toggle_column(table_column),
    }
}

fn explorer_paste(pasted: &str, state: &mut State) {
    let pasted_lines: Vec<PathBuf> = pasted.lines().map(PathBuf::from).filter(|path| path.exists()).collect();
    state.add_files(&pasted_lines);
}

fn pick_files(state: &mut State, sender: Sender<Action>) {
    state.async_action = AsyncAction::FileDialog;
    std::thread::spawn(move || {
        let task = rfd::AsyncFileDialog::new().pick_files();
        futures::executor::block_on(async {
            if let Some(files) = task.await {
                sender
                    .send(Action::Files(FileAction::AddFiles(
                        files
                            .iter()
                            .map(|f| f.path().to_path_buf())
                            .filter(|f| f.is_file())
                            .collect(),
                    )))
                    .expect("The receiver should always be available");
            }
        });
    });
}

fn pick_folders(state: &mut State, sender: Sender<Action>) {
    state.async_action = AsyncAction::FileDialog;
    std::thread::spawn(move || {
        let task = rfd::AsyncFileDialog::new().pick_folders();
        futures::executor::block_on(async {
            if let Some(folders) = task.await {
                sender
                    .send(Action::Files(FileAction::AddFolders(
                        folders.iter().map(|f| f.path().to_path_buf()).collect(),
                    )))
                    .expect("The receiver should always be available");
            }
        });
    });
}

fn add_wildcard(wildcard: &str, state: &mut State) {
    if let Ok(paths) = glob::glob(wildcard) {
        let list = paths.filter_map(Result::ok).collect::<Vec<_>>();
        state.add_files(&get_files(&list));
    }
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
