//! Kakeibo Result Note Types

use crate::types::currency::{Currency, JPY, SATS};

pub struct Note {
    queries: Vec<AnyQuery>,
}

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

pub struct Query<C>
where
    C: Currency + ?Sized,
{
    pub name: String,
    pub debit: Box<C>,
    pub credit: Box<C>,
    pub total: Box<C>,
}

impl<C> Query<C>
where
    C: Currency,
{
    pub fn create(name: String, debit: C, credit: C) -> Self {
        let total: C = debit - credit;

        Self {
            name,
            debit: Box::new(debit),
            credit: Box::new(credit),
            total: Box::new(total),
        }
    }

    pub fn new(name: String, debit: Box<C>, credit: Box<C>, total: Box<C>) -> Self {
        Self {
            name,
            debit,
            credit,
            total,
        }
    }
}
