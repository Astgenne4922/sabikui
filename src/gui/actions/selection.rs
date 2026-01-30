use std::ops::RangeInclusive;

use egui::{Pos2, Vec2};
use egui_dnd::DragUpdate;

use crate::gui::data::{hashed_file::HashedFile, state::State, table_columns::TableColumns};

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

pub fn handle(state: &mut State, action: &SelectionAction) {
    match action {
        SelectionAction::SelectAll => state.select_all(),
        SelectionAction::DeselectAll => state.deselect_all(),
        SelectionAction::CopySelected => {
            state.to_copy = Some(copy_selected(
                state.files(),
                state.selected_rows(),
                &state.active_columns(),
            ));
        }
        SelectionAction::SaveSelected => save_selected(state.files(), state.selected_rows(), &state.active_columns()),
        SelectionAction::ClearSelected => state.clear_selected(),
        SelectionAction::ClearAll => state.clear_all(),
        SelectionAction::SelectSingleRow(index) => state.select_single_row(*index),
        SelectionAction::AddRowToSelection(index) => state.add_row_to_selection(*index),
        SelectionAction::SelectRowRange(range) => state.select_range(range.clone()),
        SelectionAction::SetDragSelectionIndex(index) => state.drag_start_index = Some(*index),
        SelectionAction::ResetDragSelectionIndex => {
            state.drag_start_index = None;
            state.deselect_all();
        }
        SelectionAction::StartDrag(start, offset) => state.drag_start = Some((*start, *offset)),
        SelectionAction::EndDrag => {
            state.drag_start = None;
            state.drag_start_index = None;
        }
        SelectionAction::DragColumn(drag_update) => state.update_column_order(drag_update.from, drag_update.to),
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

fn save_selected(files: &[HashedFile], selected_rows: &[usize], columns: &[TableColumns]) {
    // TODO async
    if let Some(path) = &mut rfd::FileDialog::new().add_filter("csv", &["csv"]).save_file() {
        path.set_extension("csv");
        let to_save = columns
            .iter()
            .map(TableColumns::to_string)
            .collect::<Vec<_>>()
            .join(",");

        std::fs::write(
            path,
            format!(
                "{to_save}\n{}",
                copy_selected(files, selected_rows, columns).replace('\t', ",")
            ),
        )
        .unwrap();
    }
}
