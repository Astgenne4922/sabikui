use crate::gui::data::{hashed_file::HashedFile, state::State, table_columns::TableColumns};

#[derive(Clone)]
pub enum OptionsAction {
    CopyProperty(TableColumns),
    SortBy(TableColumns),
    ToggleAlwaysOnTop,
    ToggleMarkSame,
}

pub fn handle(state: &mut State, action: &OptionsAction) {
    match action {
        OptionsAction::CopyProperty(col) => {
            state.to_copy = Some(copy_property(col, state.files(), state.selected_rows()));
        }
        OptionsAction::SortBy(column) => state.sort_table(column.clone()),
        OptionsAction::ToggleAlwaysOnTop => state.toggle_always_on_top(),
        OptionsAction::ToggleMarkSame => state.toggle_mark_same(),
    }
}

fn copy_property(col: &TableColumns, files: &[HashedFile], selected_rows: &[usize]) -> String {
    selected_rows
        .iter()
        .map(|i| files[*i].get_from_column(col))
        .collect::<Vec<_>>()
        .join("\n")
}
