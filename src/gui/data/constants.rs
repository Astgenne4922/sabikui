pub mod labels {
    pub const FILE_MENU: &str = "File";
    pub const ADD_FILE: &str = "Add File";
    pub const ADD_FOLDER: &str = "Add Folder";
    pub const ADD_WILDCARD: &str = "Add by Wildcard";
    pub const SAVE_SELECTED: &str = "Save Selected";
    pub const CLEAR_ALL: &str = "Clear All";
    pub const CLEAR_SELECTED: &str = "Clear Selected";
    pub const EXIT: &str = "Exit";

    pub const EDIT_MENU: &str = "Edit";
    pub const COPY_SELECTED: &str = "Copy Selected";
    pub const EXPLORER_PASTE: &str = "Explorer Paste";
    pub const COPY: &str = "Copy";
    pub const SELECT_ALL: &str = "Select All";
    pub const DESELECT_ALL: &str = "Deselect All";

    pub const VIEW_MENU: &str = "View";
    pub const SORT_BY: &str = "Sort By";
    pub const FULL_PATH: &str = "Full Path";
    pub const FILENAME: &str = "Filename";
    pub const ALGORITHM: &str = "Algorithm";
    pub const EDIT_TIME: &str = "Edit Time";
    pub const FILE_SIZE: &str = "File Size (Bytes)";
    pub const EXTENSION: &str = "Extension";
    pub const REFRESH: &str = "Refresh";

    pub const OPTIONS_MENU: &str = "Options";
    pub const CHOOSE_COLUMNS: &str = "Choose Columns";
    pub const HIGHLIGHT: &str = "Highlight identical Hashes";
    pub const ALWAYS_ON_TOP: &str = "Always on Top";

    pub const HELP_MENU: &str = "Help";
    pub const ABOUT: &str = "About";
    pub const GITHUB: &str = "Github Page";
}

pub mod icons {
    pub const SAVE_SELECTED: &str = "󰆓";
    pub const REFRESH: &str = "󰑐";
    pub const ADD_FILE: &str = "";
    pub const ADD_FOLDER: &str = "󰉗";
    pub const ADD_WILDCARD: &str = "";
    pub const SELECT_ALL: &str = "󰾂";
    pub const COPY_SELECTED: &str = "󰆏";
    pub const CLEAR_ALL: &str = "󰗨";
    pub const ASCENDING: &str = "";
    pub const DESCENDING: &str = "";
}

pub mod shortcuts {
    use egui::{Key, KeyboardShortcut, Modifiers};

    pub const ADD_FILE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F2);
    pub const ADD_FOLDER: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F3);
    pub const ADD_WILDCARD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F4);
    pub const SAVE_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);
    pub const CLEAR_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::X);
    pub const CLEAR_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::Delete);

    pub const COPY_SELECTED: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::C);
    pub const EXPLORER_PASTE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::V);
    pub const SELECT_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::A);
    pub const DESELECT_ALL: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::D);

    pub const REFRESH: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F5);
}
