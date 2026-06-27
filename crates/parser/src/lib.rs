//! Kakei parser

use kakei_types::currency::Currency;
use kakei_types::query::Query;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {}

pub fn parse<C>(src: &str, currency: C) -> Result<Query<C>, ParseError>
where
    C: Currency,
{
    todo!()
}
