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

    pub fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fits(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
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

impl Filter {
    pub fn fits(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }
}
