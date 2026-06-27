//! Kakeibo Raw Note Types
//! This type is needed to express the received sheet's type.

use crate::types::currency::Currency;

/// Raw Kakeibo Note.
///
/// This type contains a `Vec<RawQuery>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawNote<C>
where
    C: Currency,
{
    queries: Vec<RawQuery<C>>,
}

impl<C> RawNote<C>
where
    C: Currency,
{
    /// Generates a `RawkakeiboNote`.
    pub fn new(queries: Vec<RawQuery<C>>) -> Self {
        Self { queries }
    }
}

/// Raw Kakeibo Query.
///
/// This type contains only query name, debit, and credit fields. This hasn't the total field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawQuery<C>
where
    C: Currency,
{
    pub name: String,
    pub debit: Box<C>,
    pub credit: Box<C>,
}

impl<C> RawQuery<C>
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
    use super::RawNote;
    use crate::types::currency::{JPY, SATS};

    #[test]
    fn raw_note() -> anyhow::Result<()> {
        // JPY
        {
            let empty_note = RawNote::<JPY>::new(vec![]);
            assert_eq!(RawNote { queries: vec![] }, empty_note);
        }

        // SATS
        {
            let empty_note = RawNote::<SATS>::new(vec![]);
            assert_eq!(RawNote { queries: vec![] }, empty_note);
        }

        Ok(())
    }
}
