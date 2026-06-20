//! Processor types

pub mod configuration;
mod currency;
mod note;

pub use configuration::*;
use std::path::PathBuf;

pub struct Processor {
    path: PathBuf,
}

impl Processor {
    pub fn read(path: PathBuf) {
        todo!()
    }

    pub fn table(&self) -> tabled::Table {
        todo!()
    }
}
