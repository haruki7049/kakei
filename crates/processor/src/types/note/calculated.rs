//! Kakeibo Raw Note Types
//! This type is needed to express the calculated sheet's type.

use super::Note;
use tabled::{Table, Tabled};

/// Calculated Kakeibo Note.
///
/// This type contains a `Vec<CalculatedKakeiboNote>`.
#[derive(Debug)]
pub struct CalculatedKakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    queries: Vec<CalculatedKakeiboQuery<C>>,
}

impl<C> CalculatedKakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    /// Generates a `CalculatedKakeiboNote`.
    pub fn new(queries: Vec<CalculatedKakeiboQuery<C>>) -> Self {
        Self { queries }
    }
}

impl<C> Note for CalculatedKakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    /// Generates a `tabled::Table` to render a ASCII table.
    fn table(&self) -> tabled::Table {
        Table::new(&self.queries)
    }
}

/// Calculated Kakeibo Query.
///
/// This type contains only query name, debit, credit, and total field.
#[derive(Debug, Tabled, Clone)]
pub struct CalculatedKakeiboQuery<C>
where
    C: std::fmt::Display + Clone,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
    pub total: C,
}

impl<C> CalculatedKakeiboQuery<C>
where
    C: std::fmt::Display + Clone,
{
    /// Generates a `CalculatedKakeiboQuery<C>`.
    pub fn new(name: String, debit: C, credit: C, total: C) -> Self {
        Self {
            name,
            debit,
            credit,
            total,
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
        let jpy_query = CalculatedKakeiboQuery {
            name: "Test JPY query".to_string(),
            debit: Currency::jpy(1),  // 1 JPY
            credit: Currency::jpy(1), // 1 JPY
            total: Currency::jpy(0),  // total: 0 JPY
        };
        let sats_query = CalculatedKakeiboQuery {
            name: "Test SATS query".to_string(),
            debit: Currency::sats(1000),  // 1000 SATS
            credit: Currency::sats(1000), // 0 JPY
            total: Currency::sats(0),     // 1000 SATS
        };
        let queries = vec![jpy_query.clone(), jpy_query, sats_query.clone(), sats_query];
        let kakeibo = CalculatedKakeiboNote::new(queries);

        let table: Table = kakeibo.table();
        assert_table!(table,
            "+-----------------+-----------+-----------+--------+"
            "| name            | debit     | credit    | total  |"
            "+-----------------+-----------+-----------+--------+"
            "| Test JPY query  | 1 JPY     | 1 JPY     | 0 JPY  |"
            "+-----------------+-----------+-----------+--------+"
            "| Test JPY query  | 1 JPY     | 1 JPY     | 0 JPY  |"
            "+-----------------+-----------+-----------+--------+"
            "| Test SATS query | 1000 SATS | 1000 SATS | 0 SATS |"
            "+-----------------+-----------+-----------+--------+"
            "| Test SATS query | 1000 SATS | 1000 SATS | 0 SATS |"
            "+-----------------+-----------+-----------+--------+"
        );

        Ok(())
    }
}
