//! Processor crate

use tabled::{Table, Tabled};

// ----- Processor -----

/// Processor trait to express the processor methods
pub trait Processor {}

#[derive(Debug)]
pub struct KakeiProcessor {}

impl Processor for KakeiProcessor {}

// ----- Kakeibo -----

pub trait Kakeibo {
    fn table(&self) -> tabled::Table;
}

#[derive(Debug)]
pub struct KakeiboNote<C>
where
    C: Unit + std::fmt::Display,
{
    queries: Vec<KakeiboQuery<C>>,
}

impl<C: Unit + std::fmt::Display> Kakeibo for KakeiboNote<C> {
    fn table(&self) -> tabled::Table {
        Table::new(&self.queries)
    }
}

#[derive(Debug, Tabled)]
pub struct KakeiboQuery<C>
where
    C: Unit + std::fmt::Display,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
}

// ----- Currency -----

pub trait Unit {
    fn is_f64(&self) -> bool;
    fn is_i64(&self) -> bool;
}

#[derive(Debug)]
pub enum Currency {
    JPY(i64),
    SATS(i64),
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JPY(_) => write!(f, "JPY"),
            Self::SATS(_) => write!(f, "SATS"),
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

// ----- Configuration -----

#[derive(Debug, Tabled)]
pub struct Configuration {}
