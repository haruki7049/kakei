//! Kakeibo Raw Note Types
//! This type is needed to express the received sheet's type.

use crate::types::currency::Currency;

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
