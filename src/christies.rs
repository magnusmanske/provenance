use crate::{event::EventKind, Event, ProvenanceSet, Reference};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Christies {}

impl Christies {
    pub fn from_html(html: &str, id: &str) -> Result<ProvenanceSet> {
        let mut ret =
            ProvenanceSet::default().with_source(crate::Identifier::Christies(id.to_string()));
        let document = Html::parse_document(html);

        let selector_accordion_item = Selector::parse("chr-accordion-item").unwrap();
        let selector_header = Selector::parse(r#"div[slot="header"]"#).unwrap();
        let selector_content = Selector::parse(r#"div[slot="content"]"#).unwrap();
        let selector_content_span =
            Selector::parse(r#"span.chr-lot-section__accordion--text"#).unwrap();
        for element in document.select(&selector_accordion_item) {
            let header = match element.select(&selector_header).next() {
                Some(div) => div.text().collect::<String>().trim().to_string(),
                None => continue,
            };
            let content_div = match element.select(&selector_content).next() {
                Some(div) => div,
                None => continue,
            };
            let content = match content_div.select(&selector_content_span).next() {
                Some(div) => div.inner_html(),
                None => continue,
            };
            if header == "Provenance" {
                Self::parse_event_section(content, &mut ret);
            } else if header == "Exhibited" {
                Self::parse_exhibition_section(content, &mut ret);
            } else if header == "Literature" {
                Self::parse_literature_section(content, &mut ret);
            }
        }
        ret.sort();

        Ok(ret)
    }

    fn parse_event_section(content: String, ret: &mut ProvenanceSet) {
        lazy_static! {
            pub static ref RE_PURCHASED: Regex = Regex::new(r"(?i)\b(purchased)\b").unwrap();
            pub static ref RE_CONFISCATED: Regex = Regex::new(r"(?i)\b(confiscated)\b").unwrap();
        }
        let parts = content.split("<br>").collect::<Vec<&str>>();
        for part in parts {
            let part = part.trim();
            let mut event = Event::default().with_text(part);
            if part.starts_with("with ") {
                event = event.with_kind(EventKind::With);
            } else if part.starts_with("Restituted ") {
                event = event.with_kind(EventKind::Restituted);
            } else if RE_PURCHASED.is_match(part) {
                event = event.with_kind(EventKind::Purchased);
            } else if RE_CONFISCATED.is_match(part) {
                event = event.with_kind(EventKind::Confiscated);
            }
            event.dates_from_text(part);
            event.locations_from_text(part);
            event.actors_from_text(part);
            ret.add_event(event);
        }
    }

    fn parse_exhibition_section(content: String, ret: &mut ProvenanceSet) {
        let parts = content.split("<br>").collect::<Vec<&str>>();
        for part in parts {
            let part = part.trim();
            let mut event = Event::default()
                .with_text(part)
                .with_kind(EventKind::Exhibited);
            event.dates_from_text(part);
            if let Some(before_comma) = part.split(',').next() {
                event.locations_from_text(before_comma);
            }
            ret.add_event(event);
        }
    }

    fn parse_literature_section(content: String, ret: &mut ProvenanceSet) {
        let parts = content.split("<br>").collect::<Vec<&str>>();
        for part in parts {
            let part = part.trim();
            let reference = Reference::from_html(part);
            ret.add_reference(reference);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Identifier;

    use super::*;

    #[test]
    fn test_christies_from_html() {
        // https://www.wikidata.org/wiki/Q102477248
        let html = include_str!("../test_files/lot-6350105");
        let res = Christies::from_html(html, "6350105").unwrap();
        assert_eq!(
            res.source(),
            Some(&Identifier::Christies("6350105".to_string()))
        );
        assert_eq!(res.events().len(), 12);
        assert_eq!(res.references().len(), 6);
    }
}
