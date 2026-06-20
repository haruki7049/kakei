//! Kakeibo Result Note Types

use crate::types::currency::{Currency, JPY, SATS};
use tabled::Tabled;

pub struct Note {
    queries: Vec<AnyQuery>,
}

impl crate::types::note::Note for self::Note {
    fn table(&self) -> tabled::Table {
        todo!()
    }
}

#[derive(Debug, Tabled)]
pub enum AnyQuery {
    JPY(Query<JPY>),
    SATS(Query<SATS>),
}

impl AnyQuery {
    pub fn jpy(jpy: Query<JPY>) -> Self {
        Self::JPY(jpy)
    }

    pub fn sats(sats: Query<SATS>) -> Self {
        Self::SATS(sats)
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
