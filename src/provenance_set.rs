use crate::{Event, Identifier, Reference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ProvenanceSet {
    source: Option<Identifier>,
    events: Vec<Event>,
    references: Vec<Reference>,
}

impl ProvenanceSet {
    pub fn with_source(mut self, source: Identifier) -> Self {
        self.source = Some(source);
        self
    }

    pub fn add_event(&mut self, mut event: Event) {
        event = event.with_order(self.events.len());
        self.events.push(event);
    }

    pub fn add_reference(&mut self, reference: Reference) {
        self.references.push(reference);
    }

    pub fn sort(&mut self) {
        self.events.sort();
    }

    pub fn source(&self) -> Option<&Identifier> {
        self.source.as_ref()
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn references(&self) -> &[Reference] {
        &self.references
    }
}
