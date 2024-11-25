use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Identifier {
    Viaf(String),
    WikidataItem(String),
    WikidataStatement(String),
    Christies(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct IdText {
    id: Option<Identifier>,
    text: Option<String>,
}

impl IdText {
    pub fn new_both(text: &str, id: &Identifier) -> Self {
        Self {
            id: Some(id.to_owned()),
            text: Some(text.to_string()),
        }
    }

    pub fn new_text(text: &str) -> Self {
        Self {
            id: None,
            text: Some(text.to_string()),
        }
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}
