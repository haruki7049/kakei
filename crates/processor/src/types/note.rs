//! Kakeibo Note Types

use tabled::{Table, Tabled};

pub trait Note {
    fn table(&self) -> tabled::Table;
}

#[derive(Debug)]
pub struct KakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    queries: Vec<KakeiboQuery<C>>,
}

impl<C> KakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    pub fn new(queries: Vec<KakeiboQuery<C>>) -> Self {
        Self { queries }
    }
}

impl<C> Note for KakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    fn table(&self) -> tabled::Table {
        Table::new(&self.queries)
    }
}

#[derive(Debug, Tabled, Clone)]
pub struct KakeiboQuery<C>
where
    C: std::fmt::Display + Clone,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
}

impl<C> KakeiboQuery<C>
where
    C: std::fmt::Display + Clone,
{
    pub fn new(name: String, debit: C, credit: C) -> Self {
        Self {
            name,
            debit,
            credit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use tabled::assert::assert_table;

    #[test]
    fn table() -> anyhow::Result<()> {
        let jpy_query = KakeiboQuery {
            name: "Test JPY query".to_string(),
            debit: Currency::jpy(1),  // 1 JPY
            credit: Currency::jpy(0), // 0 JPY
        };
        let sats_query = KakeiboQuery {
            name: "Test SATS query".to_string(),
            debit: Currency::sats(1000), // 1000 JPY
            credit: Currency::sats(0),   // 0 JPY
        };
        let queries = vec![jpy_query.clone(), jpy_query, sats_query.clone(), sats_query];
        let kakeibo = KakeiboNote::new(queries);

        let table: Table = kakeibo.table();
        assert_table!(
            table,
            "+-----------------+-----------+--------+"
            "| name            | debit     | credit |"
            "+-----------------+-----------+--------+"
            "| Test JPY query  | 1 JPY     | 0 JPY  |"
            "+-----------------+-----------+--------+"
            "| Test JPY query  | 1 JPY     | 0 JPY  |"
            "+-----------------+-----------+--------+"
            "| Test SATS query | 1000 SATS | 0 SATS |"
            "+-----------------+-----------+--------+"
            "| Test SATS query | 1000 SATS | 0 SATS |"
            "+-----------------+-----------+--------+"
        );

        Ok(())
    }
}
