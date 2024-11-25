use csv::ReaderBuilder;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::Identifier;

lazy_static! {
    pub static ref CITIES: Vec<(Identifier, String, Regex)> = Location::load_locations();
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Location {}

#[derive(Debug, Deserialize)]
struct WikiCity {
    label: String,
    qid: String,
}

impl Location {
    // Loads City Qid / label / Regex list
    // Called only once on startup
    fn load_locations() -> Vec<(Identifier, String, Regex)> {
        let tsv_data = include_str!("../static/cities.tsv");
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t') // Set the delimiter to tab
            .from_reader(tsv_data.as_bytes());
        let mut duplicate_keys: HashSet<String> = HashSet::new();
        let mut ret = HashMap::new();
        for result in rdr.deserialize() {
            let city: WikiCity = result.unwrap();
            if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(city.label.to_owned()) {
                e.insert(Identifier::WikidataItem(city.qid));
            } else {
                duplicate_keys.insert(city.label.to_owned());
            }
        }
        for key in duplicate_keys {
            ret.remove(&key);
        }

        ret.into_iter()
            .map(|(label, id)| {
                let re = Regex::new(&format!(r"\b{}\b", label)).unwrap();
                (id, label, re)
            })
            .collect()
    }
}
