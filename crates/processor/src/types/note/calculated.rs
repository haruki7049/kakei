//! Kakeibo Raw Note Types
//! This type is needed to express the calculated sheet's type.

use crate::types::{currency::Unit, note::Note};
use tabled::{Table, Tabled};

/// Calculated Kakeibo Note.
///
/// This type contains a `Vec<CalculatedKakeiboNote>`.
#[derive(Debug)]
pub struct CalculatedKakeiboNote<C>
where
    C: Currency + Clone,
{
    queries: Vec<CalculatedKakeiboQuery<C>>,
}

impl<C> CalculatedKakeiboNote<C>
where
    C: Currency + Clone,
{
    /// Generates a `CalculatedKakeiboNote`.
    pub fn new(queries: Vec<CalculatedKakeiboQuery<C>>) -> Self {
        Self { queries }
    }
}

impl<C> Note for CalculatedKakeiboNote<C>
where
    C: Currency + Clone,
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
pub struct CalculatedKakeiboQuery {
    pub name: String,
    pub debit: Unit,
    pub credit: Unit,
    pub total: Unit,
}

impl CalculatedKakeiboQuery {
    /// Generates a `CalculatedKakeiboQuery`.
    pub fn new(name: String, debit: Unit, credit: Unit, total: Unit) -> Self {
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
    use crate::types::currency::Currency;
    use crate::types::currency::JPY;
    use crate::types::currency::SATS;
    use crate::types::note::calculated::CalculatedKakeiboNote;
    use crate::types::note::calculated::CalculatedKakeiboQuery;
    use tabled::assert::assert_table;
    use tabled::{Table, Tabled};

    #[test]
    fn table() -> anyhow::Result<()> {
        let jpy_query = CalculatedKakeiboQuery {
            name: "Test JPY query".to_string(),
            debit: JPY::new(1),  // 1 JPY
            credit: JPY::new(1), // 1 JPY
            total: JPY::new(0),  // total: 0 JPY
        };
        let sats_query = CalculatedKakeiboQuery {
            name: "Test SATS query".to_string(),
            debit: SATS::new(1000),  // 1000 SATS
            credit: SATS::new(1000), // 0 JPY
            total: SATS::new(0),     // 1000 SATS
        };
        let queries: Vec<CalculatedKakeiboQuery> =
            vec![jpy_query.clone(), jpy_query, sats_query.clone(), sats_query];
        let kakeibo: CalculatedKakeiboNote<dyn Currency> = CalculatedKakeiboNote::new(queries);

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
