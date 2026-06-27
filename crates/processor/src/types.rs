//! Processor types

pub mod configuration;
mod currency;
mod note;

use crate::types::{
    currency::{JPY, SATS},
    note::{Note as _, result::Note},
};
pub use configuration::*;
use std::path::PathBuf;

pub struct Processor {
    jpy_notes: Vec<Note<JPY>>,
    sats_notes: Vec<Note<SATS>>,
}

impl Processor {
    pub fn read(paths: Vec<PathBuf>) {
        todo!()
    }

    pub fn tables(&self) -> Vec<tabled::Table> {
        let mut result: Vec<tabled::Table> = Vec::new();

        // JPY
        for jpy_note in self.jpy_notes.iter() {
            let table: tabled::Table = jpy_note.table();
            result.push(table);
        }

        // SATS
        for sats_note in self.sats_notes.iter() {
            let table: tabled::Table = sats_note.table();
            result.push(table);
        }

        result
    }
}
