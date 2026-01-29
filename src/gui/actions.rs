use std::{
    collections::VecDeque,
    fs,
    ops::RangeInclusive,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};

use egui::{Pos2, Vec2};
use egui_dnd::DragUpdate;

use crate::gui::data::{
    hashed_file::HashedFile,
    state::{AsyncAction, OpenWindow, State},
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
    PickFiles,
    PickFolders,
    AddFiles(Vec<PathBuf>),
    AddFolders(Vec<PathBuf>),
    AddWildcard(String),
    CopyProperty(TableColumns),
    SortBy(TableColumns),
    ToggleColumn(TableColumns),
    ToggleAlwaysOnTop,
    ToggleMarkSame,
    SelectSingleRow(usize),
    AddRowToSelection(usize),
    SelectRowRange(RangeInclusive<usize>),
    SetDragSelectionIndex(usize),
    ResetDragSelectionIndex,
    StartDrag(Pos2, Vec2),
    EndDrag,
    DragColumn(DragUpdate),
    CloseExternalWindow,
    OpenExternalWindow(OpenWindow),
}

pub struct ActionHandler {
    message_receiver: Receiver<Action>,
}

impl ActionHandler {
    pub const fn new(message_receiver: Receiver<Action>) -> Self {
        Self { message_receiver }
    }

    pub fn handle(&self, state: &mut State, sender: &Sender<Action>) {
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
                Action::PickFiles => pick_files(state, sender.clone()),
                Action::PickFolders => pick_folders(state, sender.clone()),
                Action::AddFiles(new_files) => {
                    state.async_action = AsyncAction::None;
                    state.add_files(&new_files);
                }
                Action::AddFolders(folders) => {
                    state.async_action = AsyncAction::None;
                    state.add_files(&get_files(&folders));
                }
                Action::AddWildcard(wildcard) => {
                    state.open_extra_window = OpenWindow::None;
                    add_wildcard(&wildcard, state);
                }
                Action::CopyProperty(col) => {
                    state.to_copy = Some(copy_property(&col, state.files(), state.selected_rows()));
                }
                Action::SortBy(column) => state.sort_table(column),
                Action::ToggleColumn(table_column) => state.toggle_column(&table_column),
                Action::ToggleAlwaysOnTop => state.toggle_always_on_top(),
                Action::ToggleMarkSame => state.toggle_mark_same(),
                Action::SelectSingleRow(index) => state.select_single_row(index),
                Action::AddRowToSelection(index) => state.add_row_to_selection(index),
                Action::SelectRowRange(range) => state.select_range(range),
                Action::StartDrag(start, offset) => state.drag_start = Some((start, offset)),
                Action::EndDrag => {
                    state.drag_start = None;
                    state.drag_start_index = None;
                }
                Action::SetDragSelectionIndex(index) => state.drag_start_index = Some(index),
                Action::ResetDragSelectionIndex => {
                    state.drag_start_index = None;
                    state.deselect_all();
                }
                Action::DragColumn(drag_update) => state.update_column_order(drag_update.from, drag_update.to),
                Action::CloseExternalWindow => {
                    state.open_extra_window = OpenWindow::None;
                }
                Action::OpenExternalWindow(open_window) => {
                    state.open_extra_window = open_window;
                }
            }
        }
    }
}

fn explorer_paste(pasted: &str, state: &mut State) {
    let pasted_lines: Vec<PathBuf> = pasted.lines().map(PathBuf::from).filter(|path| path.exists()).collect();
    state.add_files(&pasted_lines);
}

fn copy_selected(files: &[HashedFile], selected_rows: &[usize], columns: &[TableColumns]) -> String {
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

fn save_selected(files: &[HashedFile], selected_rows: &[usize], columns: &[TableColumns]) {
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

fn pick_files(state: &mut State, sender: Sender<Action>) {
    state.async_action = AsyncAction::FileDialog;
    std::thread::spawn(move || {
        let task = rfd::AsyncFileDialog::new().pick_files();
        futures::executor::block_on(async {
            if let Some(files) = task.await {
                sender
                    .send(Action::AddFiles(
                        files
                            .iter()
                            .map(|f| f.path().to_path_buf())
                            .filter(|f| f.is_file())
                            .collect(),
                    ))
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
                    .send(Action::AddFolders(
                        folders.iter().map(|f| f.path().to_path_buf()).collect(),
                    ))
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

fn copy_property(col: &TableColumns, files: &[HashedFile], selected_rows: &[usize]) -> String {
    selected_rows
        .iter()
        .map(|i| files[*i].get_from_column(col))
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
