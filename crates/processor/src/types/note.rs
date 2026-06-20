pub mod raw;
pub mod result;

/// Note trait to convert to `tabled::Table`.
pub trait Note {
    fn table(&self) -> tabled::Table;
}
