//! Processor crate

pub mod configuration;
mod currency;
mod note;

use crate::{
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

#[cfg(test)]
mod tests {
    use crate::Processor;
    use tabled::{Table, assert::assert_table};

    #[test]
    fn empty_tables() -> anyhow::Result<()> {
        let processor = Processor {
            jpy_queries: vec![],
            sats_queries: vec![],
        };
        let tables: Vec<Table> = processor.tables();

        assert_eq!(tables.len(), 2);
        assert_table!(
            tables[0],
            "+------+-------+--------+-------+"
            "| name | debit | credit | total |"
            "+------+-------+--------+-------+"
        );
        assert_table!(
            tables[1],
            "+------+-------+--------+-------+"
            "| name | debit | credit | total |"
            "+------+-------+--------+-------+"
        );
        Ok(())
    }
}
