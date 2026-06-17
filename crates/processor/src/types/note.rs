pub mod calculated;
pub mod raw;

/// Note trait to convert to `tabled::Table`.
pub trait Note {
    fn table(&self) -> tabled::Table;
}
