use crate::currency::Currency;
use tabled::Tabled;

#[derive(Debug, Tabled, Clone)]
pub struct Query<C>
where
    C: Currency,
{
    pub name: String,
    pub debit: C,
    pub credit: C,
    pub total: C,
}
