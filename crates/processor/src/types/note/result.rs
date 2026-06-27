//! Kakeibo Result Note Types

use crate::types::currency::Currency;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Query<C>
where
    C: Currency,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
    pub total: C,
}

impl<C> Query<C>
where
    C: Currency,
{
    pub fn create(name: String, debit: C, credit: C) -> Self {
        let total: C = debit - credit;

        Self {
            name,
            debit,
            credit,
            total,
        }
    }

    pub fn new(name: String, debit: C, credit: C, total: C) -> Self {
        Self {
            name,
            debit,
            credit,
            total,
        }
    }
}
