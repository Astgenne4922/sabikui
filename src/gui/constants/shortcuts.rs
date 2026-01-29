use egui::{Key, KeyboardShortcut, Modifiers};

use crate::gui::{actions::Action, data::state::OpenWindow};

pub const ADD_FILE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F1);
pub const ADD_FOLDER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F2);
pub const ADD_WILDCARD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F3);
pub const SAVE_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
pub const CLEAR_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::X);
pub const CLEAR_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::Delete);

pub const COPY_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::C);
pub const EXPLORER_PASTE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::V);
pub const SELECT_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::A);
pub const DESELECT_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::D);

pub const REFRESH: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F5);

pub const KEYBINDS: [(&KeyboardShortcut, Action); 8] = [
    (&SELECT_ALL, Action::SelectAll),
    (&DESELECT_ALL, Action::DeselectAll),
    (&SAVE_SELECTED, Action::SaveSelected),
    (&ADD_FILE, Action::PickFiles),
    (&ADD_FOLDER, Action::PickFolders),
    (&ADD_WILDCARD, Action::OpenExternalWindow(OpenWindow::AddWildcard)),
    (&REFRESH, Action::Refresh),
    (&CLEAR_SELECTED, Action::ClearSelected),
];
