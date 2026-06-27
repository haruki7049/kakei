//! Processor types

pub mod configuration;
mod currency;
mod note;

use crate::types::{
    currency::{JPY, SATS},
    note::Query,
};
pub use configuration::*;
use std::path::PathBuf;
use tabled::Table;

pub struct Processor {
    jpy_queries: Vec<Query<JPY>>,
    sats_queries: Vec<Query<SATS>>,
}

impl Processor {
    pub fn read(paths: Vec<PathBuf>) {
        todo!()
    }

    pub fn tables(&self) -> Vec<tabled::Table> {
        let mut result: Vec<tabled::Table> = Vec::new();

        // JPY
        let jpy_table = Table::new(self.jpy_queries.clone());
        result.push(jpy_table);

        // SATS
        let sats_table = Table::new(self.sats_queries.clone());
        result.push(sats_table);

        result
    }
}
