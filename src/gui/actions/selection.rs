use std::{
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

use egui::{Pos2, Vec2};
use egui_dnd::DragUpdate;

use crate::gui::data::{
    hashed_file::HashedFile,
    state::{AsyncAction, State},
    table_columns::TableColumns,
};

#[derive(Clone)]
pub enum SelectionAction {
    SelectAll,
    DeselectAll,
    CopySelected,
    SaveSelected,
    ClearSelected,
    ClearAll,
    SelectSingleRow(usize),
    AddRowToSelection(usize),
    SelectRowRange(RangeInclusive<usize>),
    SetDragSelectionIndex(usize),
    ResetDragSelectionIndex,
    StartDrag(Pos2, Vec2),
    EndDrag,
    DragColumn(DragUpdate),
}

pub fn handle(state: Arc<Mutex<State>>, action: &SelectionAction) {
    match action {
        SelectionAction::SelectAll => State::lock(&state).select_all(),
        SelectionAction::DeselectAll => State::lock(&state).deselect_all(),
        SelectionAction::CopySelected => {
            let mut lock = State::lock(&state);
            lock.to_copy = Some(copy_selected(
                lock.files(),
                lock.selected_rows(),
                &lock.active_columns(),
            ));
        }
        SelectionAction::SaveSelected => save_selected(state),
        SelectionAction::ClearSelected => State::lock(&state).clear_selected(),
        SelectionAction::ClearAll => State::lock(&state).clear_all(),
        SelectionAction::SelectSingleRow(index) => {
            State::lock(&state).select_single_row(*index);
        }
        SelectionAction::AddRowToSelection(index) => State::lock(&state).add_row_to_selection(*index),
        SelectionAction::SelectRowRange(range) => {
            State::lock(&state).select_range(range.clone());
        }
        SelectionAction::SetDragSelectionIndex(index) => {
            State::lock(&state).drag_start_index = Some(*index);
        }
        SelectionAction::ResetDragSelectionIndex => {
            let mut lock = State::lock(&state);
            lock.drag_start_index = None;
            lock.deselect_all();
        }
        SelectionAction::StartDrag(start, offset) => {
            State::lock(&state).drag_start = Some((*start, *offset));
        }
        SelectionAction::EndDrag => {
            let mut lock = State::lock(&state);
            lock.drag_start = None;
            lock.drag_start_index = None;
        }
        SelectionAction::DragColumn(drag_update) => {
            State::lock(&state).update_column_order(drag_update.from, drag_update.to);
        }
    }
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

fn save_selected(state: Arc<Mutex<State>>) {
    State::lock(&state).async_action = AsyncAction::FileDialog;

    std::thread::spawn(move || {
        if let Some(path) = &mut rfd::FileDialog::new().add_filter("csv", &["csv"]).save_file() {
            path.set_extension("csv");

            let mut lock = State::lock(&state);
            let to_save = lock
                .active_columns()
                .iter()
                .map(TableColumns::to_string)
                .collect::<Vec<_>>()
                .join(",");

            std::fs::write(
                path,
                format!(
                    "{to_save}\n{}",
                    copy_selected(lock.files(), lock.selected_rows(), &lock.active_columns()).replace('\t', ",")
                ),
            )
            .expect("The save path should be writable");

            lock.async_action = AsyncAction::None;
        }
    });
}
