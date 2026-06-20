//! Currency module

use std::ops::{Add, Sub};

pub trait Currency: Add<Output = Self> + Sub<Output = Self> + Sized + Copy {}

// ----- Each Currency Unit -----

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JPY {
    inner: i64,
}

impl Currency for JPY {}

impl JPY {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for JPY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} JPY", self.inner)
    }
}

impl std::ops::Add for JPY {
    type Output = JPY;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.inner + rhs.inner;
        Self { inner }
    }
}

impl std::ops::Sub for JPY {
    type Output = JPY;

    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.inner - rhs.inner;
        Self { inner }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SATS {
    inner: i64,
}

impl Currency for SATS {}

impl SATS {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for SATS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} SATS", self.inner)
    }
}

impl std::ops::Add for SATS {
    type Output = SATS;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.inner + rhs.inner;
        Self { inner }
    }
}

impl std::ops::Sub for SATS {
    type Output = SATS;

    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.inner - rhs.inner;
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency() -> anyhow::Result<()> {
        let _ = JPY::new(1); // 1 JPY
        let _ = SATS::new(1000); // 1000 SATS
        Ok(())
    }
}
