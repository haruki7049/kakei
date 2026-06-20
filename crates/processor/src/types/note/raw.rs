//! Kakeibo Raw Note Types
//! This type is needed to express the received sheet's type.

use crate::prelude::*;

/// Raw Kakeibo Note.
///
/// This type contains a `Vec<RawkakeiboQuery>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawKakeiboNote<C>
where
    C: Currency + ?Sized,
{
    queries: Vec<RawKakeiboQuery<C>>,
}

impl<C> RawKakeiboNote<C>
where
    C: Currency,
{
    /// Generates a `RawkakeiboNote`.
    pub fn new(queries: Vec<RawKakeiboQuery<C>>) -> Self {
        Self { queries }
    }
}

/// Raw Kakeibo Query.
///
/// This type contains only query name, debit, and credit fields. This hasn't the total field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawKakeiboQuery<C>
where
    C: Currency + ?Sized,
{
    pub name: String,
    pub debit: Box<C>,
    pub credit: Box<C>,
}

impl<C> RawKakeiboQuery<C>
where
    C: Currency,
{
    /// Generates a `RawKakeiboQuery<C>`.
    pub fn new(name: String, debit: C, credit: C) -> Self {
        Self {
            name,
            debit: Box::new(debit),
            credit: Box::new(credit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table() -> anyhow::Result<()> {
        let jpy_query = RawKakeiboQuery {
            name: "Test JPY query".to_string(),
            debit: Box::new(JPY::new(1)),  // 1 JPY
            credit: Box::new(JPY::new(0)), // 0 JPY
        };
        let sats_query = RawKakeiboQuery {
            name: "Test SATS query".to_string(),
            debit: Box::new(SATS::new(1000)), // 1000 SATS
            credit: Box::new(SATS::new(0)),   // 0 SATS
        };
        let queries: Vec<RawKakeiboQuery<dyn Currency>> = vec![
            jpy_query.clone(),
            jpy_query.clone(),
            sats_query.clone(),
            sats_query.clone(),
        ];
        // let kakeibo: RawKakeiboNote<dyn Currency> = RawKakeiboNote::new(queries);

        // assert_eq!(
        //     kakeibo,
        //     RawKakeiboNote {
        //         queries: vec![jpy_query.clone(), jpy_query, sats_query.clone(), sats_query]
        //     }
        // );

        todo!()
        // Ok(())
    }
}
