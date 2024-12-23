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
        let id = if text.to_lowercase().trim() == "gestapo" {
            Some(Identifier::WikidataItem("Q43250".to_string()))
        } else {
            None
        };

        Self {
            id,
            text: Some(text.to_string()),
        }
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_text() {
        let id_text = IdText::new_both("text", &Identifier::Christies("id".to_string()));
        assert_eq!(id_text.text(), Some("text"));
    }

    #[test]
    fn test_id_text_no_id() {
        let id_text = IdText::new_text("text");
        assert_eq!(id_text.text(), Some("text"));
    }

    #[test]
    fn test_id_text_no_text() {
        let id_text = IdText {
            id: Some(Identifier::Christies("id".to_string())),
            text: None,
        };
        assert_eq!(id_text.text(), None);
    }
}
