//! Processor crate

use tabled::{Table, Tabled};

// ----- Processor -----

/// Processor trait to express the processor methods
pub trait Processor {}

#[derive(Debug)]
pub struct KakeiProcessor {}

impl Processor for KakeiProcessor {}

// ----- Kakeibo -----

pub trait Note {
    fn table(&self) -> tabled::Table;
}

#[derive(Debug)]
pub struct KakeiboNote<C>
where
    C: Unit + std::fmt::Display + Clone,
{
    queries: Vec<KakeiboQuery<C>>,
}

impl<C> KakeiboNote<C>
where
    C: Unit + std::fmt::Display + Clone,
{
    pub fn new(queries: Vec<KakeiboQuery<C>>) -> Self {
        Self { queries }
    }
}

impl<C> Note for KakeiboNote<C>
where
    C: Unit + std::fmt::Display + Clone,
{
    fn table(&self) -> tabled::Table {
        Table::new(&self.queries)
    }
}

#[derive(Debug, Tabled, Clone)]
pub struct KakeiboQuery<C>
where
    C: Unit + std::fmt::Display + Clone,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
}

impl<C> KakeiboQuery<C>
where
    C: Unit + std::fmt::Display + Clone,
{
    pub fn new(name: String, debit: C, credit: C) -> Self {
        Self {
            name,
            debit,
            credit,
        }
    }
}

// ----- Currency & Unit -----

pub trait Unit {
    fn is_f64(&self) -> bool;
    fn is_i64(&self) -> bool;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Currency {
    JPY(JPY),
    SATS(SATS),
}

impl Currency {
    pub fn jpy(inner: i64) -> Self {
        Self::JPY(JPY { inner })
    }

    pub fn sats(inner: i64) -> Self {
        Self::SATS(SATS { inner })
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JPY(i) => write!(f, "{}", i),
            Self::SATS(i) => write!(f, "{}", i),
        }
    }
}

impl Unit for Currency {
    fn is_f64(&self) -> bool {
        match self {
            Self::JPY(_) => false,
            Self::SATS(_) => false,
        }
    }

    fn is_i64(&self) -> bool {
        match self {
            Self::JPY(_) => true,
            Self::SATS(_) => true,
        }
    }
}

// ----- Each Currency Unit -----

#[derive(Debug, PartialEq, Clone)]
pub struct JPY {
    inner: i64,
}

impl std::fmt::Display for JPY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} JPY", self.inner)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SATS {
    inner: i64,
}

impl std::fmt::Display for SATS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} SATS", self.inner)
    }
}

// ----- Configuration -----

#[derive(Debug, Tabled)]
pub struct Configuration {}

#[cfg(test)]
mod tests {
    use super::*;
    use tabled::assert::assert_table;

    #[test]
    fn currency() -> anyhow::Result<()> {
        let _ = Currency::jpy(1); // 1 JPY
        let _ = Currency::sats(1000); // 1000 SATS
        Ok(())
    }

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
