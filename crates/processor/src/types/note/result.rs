//! Kakeibo Result Note Types

use crate::types::{
    currency::{Currency, JPY, SATS},
    note::raw::RawQuery,
};
use tabled::Tabled;

pub struct Note<C>
where
    C: Currency,
{
    queries: Vec<Query<C>>,
}

impl<C> crate::types::note::Note for self::Note<C>
where
    C: Currency,
{
    fn table(&self) -> tabled::Table {
        todo!()
    }
}

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

impl<C> From<RawQuery<C>> for Query<C>
where
    C: Currency,
{
    fn from(value: RawQuery<C>) -> Self {
        let name = value.name;
        let debit = *value.debit;
        let credit = *value.credit;
        let total = *value.debit - *value.credit;

        Self {
            name,
            debit,
            credit,
            total,
        }
    }
}
