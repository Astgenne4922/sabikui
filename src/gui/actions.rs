use std::sync::mpsc;

use crate::gui::data::hashed_file::{HashFunction, HashedFile};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Action {
    Refresh,
    Paste,
    SelectAll,
    DeselectAll,
    CopySelected,
    SaveSelected,
    ClearSelected,
    ClearAll,
    AddFile,
    AddFolder,
    AddWildcard,
    CopyHash(String),
    SortBy(String),
}

pub struct ActionHandler {
    message_receiver: mpsc::Receiver<Action>,
}

impl ActionHandler {
    pub fn new(message_receiver: mpsc::Receiver<Action>) -> Self {
        Self { message_receiver }
    }

    pub fn handle(&self, files: &mut [HashedFile], algorithms: &[HashFunction]) {
        while let Ok(action) = self.message_receiver.try_recv() {
            match action {
                Action::Refresh => refresh(files, algorithms),
                Action::Paste => explorer_paste(),
                Action::SelectAll => select_all(),
                Action::DeselectAll => deselect_all(),
                Action::CopySelected => copy_selected(),
                Action::SaveSelected => save_selected(),
                Action::ClearSelected => clear_selected(),
                Action::ClearAll => clear_all(),
                Action::AddFile => add_file(),
                Action::AddFolder => add_folder(),
                Action::AddWildcard => add_wildcard(),
                Action::CopyHash(_) => copy_hash(),
                Action::SortBy(_) => sort_by(),
            }
        }
    }
}

// TODO Refresh - F5
fn refresh(files: &mut [HashedFile], algorithms: &[HashFunction]) {
    for file in files {
        *file = HashedFile::new(&file.path, algorithms);
    }
}
// TODO Explorer paste - CTRL+V
fn explorer_paste() {
    println!("EXPLORER PASTE");
}

// TODO select all - CTRL+A
fn select_all() {
    println!("SELECT ALL");
}
// TODO deselect all - CTRL+D
fn deselect_all() {
    println!("DESELECT ALL");
}
// TODO Copy Selected - CTRL+C
fn copy_selected() {
    println!("COPY SELECTED");
}
// TODO Save selected - CTRL+S
fn save_selected() {
    println!("SAVE SELECTED");
}
// TODO clear selected - Del
fn clear_selected() {
    println!("CLEAR SELECTED");
}
// TODO clear all - CTRL+X
fn clear_all() {
    println!("CLEAR ALL");
}

// TODO add file - F2
fn add_file() {
    println!("ADD FILE");
}
// TODO add folder - F3
fn add_folder() {
    println!("ADD FOLDER");
}
// TODO add wildcard - F4
fn add_wildcard() {
    println!("ADD WILDCARD");
}

// TODO Copy [ALG]
fn copy_hash() {
    println!("COPY HASH");
}

// TODO Sort By
fn sort_by() {
    println!("SORT BY");
}
