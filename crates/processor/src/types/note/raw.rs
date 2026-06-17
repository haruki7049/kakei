//! Kakeibo Raw Note Types
//! This type is needed to express the received sheet's type.

/// Raw Kakeibo Note.
///
/// This type contains a `Vec<RawkakeiboQuery>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawKakeiboNote<C>
where
    C: std::fmt::Display + Clone,
{
    queries: Vec<RawKakeiboQuery<C>>,
}

impl<C> RawKakeiboNote<C>
where
    C: std::fmt::Display + Clone,
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
    C: std::fmt::Display + Clone,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
}

impl<C> RawKakeiboQuery<C>
where
    C: std::fmt::Display + Clone,
{
    /// Generates a `RawKakeiboQuery<C>`.
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

    #[test]
    fn table() -> anyhow::Result<()> {
        let jpy_query = RawKakeiboQuery {
            name: "Test JPY query".to_string(),
            debit: Currency::jpy(1),  // 1 JPY
            credit: Currency::jpy(0), // 0 JPY
        };
        let sats_query = RawKakeiboQuery {
            name: "Test SATS query".to_string(),
            debit: Currency::sats(1000), // 1000 JPY
            credit: Currency::sats(0),   // 0 JPY
        };
        let queries = vec![
            jpy_query.clone(),
            jpy_query.clone(),
            sats_query.clone(),
            sats_query.clone(),
        ];
        let kakeibo = RawKakeiboNote::new(queries);

        assert_eq!(
            kakeibo,
            RawKakeiboNote {
                queries: vec![jpy_query.clone(), jpy_query, sats_query.clone(), sats_query]
            }
        );
        Ok(())
    }
}
