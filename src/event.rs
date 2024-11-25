use crate::{Date, IdText, Identifier, Reference};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum EventKind {
    #[default]
    Unknown,
    Owned,
    With,
    Purchased,
    Confiscated,
    Restituted,
    Exhibited,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Event {
    id: Option<Identifier>,
    kind: EventKind,
    order: usize,
    start: Option<Date>,
    end: Option<Date>,
    text: Option<String>,
    locations: Vec<IdText>,
    actors: Vec<IdText>,
    references: Vec<Reference>,
}

impl Event {
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn with_order(mut self, number: usize) -> Self {
        self.order = number;
        self
    }

    pub fn with_kind(mut self, kind: EventKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn actors_from_text(&mut self, text: &str) {
        let mut text = text.to_owned();
        lazy_static! {
            pub static ref RE_NAME: Regex = Regex::new(
                r"\b([A-Z]\. [A-Z]\S+|[A-Z]\S+ [A-Z]\S+|[A-Z]\S+ von [A-Z]\S+|Gestapo)\b"
            )
            .unwrap();
            pub static ref RE_COUNTER_INDICATION: Regex =
                Regex::new(r"\b(Galerie|Galery|Co|New)\b").unwrap();
        }

        // Replace locations in text
        for location in &self.locations {
            if let Some(label) = location.text() {
                text = text.replace(label, " ");
            }
        }

        self.actors = RE_NAME
            .find_iter(&text)
            .filter(|m| !RE_COUNTER_INDICATION.is_match(m.as_str()))
            .map(|m| IdText::new_text(m.as_str()))
            .collect();
    }

    pub fn locations_from_text(&mut self, text: &str) {
        self.locations = crate::location::CITIES
            .par_iter()
            .filter(|(_id, _label, re)| re.is_match(text))
            .map(|(id, label, _re)| IdText::new_both(label, id))
            .collect();
    }

    pub fn dates_from_text(&mut self, text: &str) {
        let mut dates = Self::parse_dates_from_text(text);
        dates.sort();
        self.start = dates.first().cloned();

        // Leave end=None if there is only one date
        if dates.len() > 1 {
            self.end = dates.last().cloned();
        }
    }

    fn parse_dates_from_text(text: &str) -> Vec<Date> {
        lazy_static! {
            pub static ref RE_YEAR: Regex = Regex::new(r"\d{3,}").unwrap();
        }
        RE_YEAR
            .split_inclusive(text)
            .par_bridge()
            .map(String::from)
            .filter_map(|part| Date::from_text(&part))
            .collect()
    }
}
