use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Reference {
    html: String,
}

impl Reference {
    pub fn from_html(html: &str) -> Self {
        Self {
            html: html.to_string(),
        }
    }
}
