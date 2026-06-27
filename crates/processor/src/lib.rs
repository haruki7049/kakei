//! Processor crate

pub mod configuration;
mod currency;
mod query;

use crate::{
    currency::{JPY, SATS},
    query::Query,
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
    use crate::{
        Processor,
        currency::{JPY, SATS},
        query::Query,
    };
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

    #[test]
    fn some_tables() -> anyhow::Result<()> {
        let processor = Processor {
            jpy_queries: vec![
                Query::<JPY> {
                    name: "Test JPY query".to_string(),
                    debit: JPY(0),
                    credit: JPY(0),
                    total: JPY(0),
                },
                Query::<JPY> {
                    name: "Test JPY query".to_string(),
                    debit: JPY(0),
                    credit: JPY(0),
                    total: JPY(0),
                },
            ],
            sats_queries: vec![
                Query::<SATS> {
                    name: "Test SATS query".to_string(),
                    debit: SATS(0),
                    credit: SATS(0),
                    total: SATS(0),
                },
                Query::<SATS> {
                    name: "Test SATS query".to_string(),
                    debit: SATS(0),
                    credit: SATS(0),
                    total: SATS(0),
                },
            ],
        };
        let tables: Vec<Table> = processor.tables();

        assert_eq!(tables.len(), 2);
        assert_table!(
            tables[0],
            "+----------------+-------+--------+-------+"
            "| name           | debit | credit | total |"
            "+----------------+-------+--------+-------+"
            "| Test JPY query | 0 JPY | 0 JPY  | 0 JPY |"
            "+----------------+-------+--------+-------+"
            "| Test JPY query | 0 JPY | 0 JPY  | 0 JPY |"
            "+----------------+-------+--------+-------+"
        );
        assert_table!(
            tables[1],
            "+-----------------+--------+--------+--------+"
            "| name            | debit  | credit | total  |"
            "+-----------------+--------+--------+--------+"
            "| Test SATS query | 0 SATS | 0 SATS | 0 SATS |"
            "+-----------------+--------+--------+--------+"
            "| Test SATS query | 0 SATS | 0 SATS | 0 SATS |"
            "+-----------------+--------+--------+--------+"
        );
        Ok(())
    }
}
