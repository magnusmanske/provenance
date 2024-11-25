pub mod christies;
pub mod date;
pub mod event;
pub mod identifier;
pub mod location;
pub mod provenance_set;
pub mod reference;

pub use crate::{
    date::Date,
    event::Event,
    identifier::{IdText, Identifier},
    provenance_set::ProvenanceSet,
    reference::Reference,
};

fn main() {
    println!("Hello, world!");
}
