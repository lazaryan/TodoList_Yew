use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub entries: Vec<Entry>,
    pub filter: Filter,
    pub edit_value: String,
}

impl State {
    pub fn total(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub description: String,
    pub completed: bool,
    pub editing: bool,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}
