use crate::gui::data::state::{OpenWindow, State};

#[derive(Clone)]
pub enum WindowAction {
    CloseExternalWindow,
    OpenExternalWindow(OpenWindow),
}

pub const fn handle(state: &mut State, action: &WindowAction) {
    match action {
        WindowAction::CloseExternalWindow => {
            state.open_extra_window = OpenWindow::None;
        }
        WindowAction::OpenExternalWindow(open_window) => {
            state.open_extra_window = *open_window;
        }
    }
}
