//! Currency module

use std::ops::{Add, Sub};

pub trait Currency:
    Add<Output = Self> + Sub<Output = Self> + Sized + Copy + std::fmt::Display
{
}

// ----- Each Currency Unit -----

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JPY(pub i64);

impl Currency for JPY {}

impl std::fmt::Display for JPY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} JPY", self.0)
    }
}

impl std::ops::Add for JPY {
    type Output = JPY;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.0 + rhs.0;
        Self(inner)
    }
}

impl std::ops::Sub for JPY {
    type Output = JPY;

    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.0 - rhs.0;
        Self(inner)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SATS(pub i64);

impl Currency for SATS {}

impl std::fmt::Display for SATS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} SATS", self.0)
    }
}

impl std::ops::Add for SATS {
    type Output = SATS;

    fn add(self, rhs: Self) -> Self::Output {
        let inner = self.0 + rhs.0;
        Self(inner)
    }
}

impl std::ops::Sub for SATS {
    type Output = SATS;

    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.0 - rhs.0;
        Self(inner)
    }
}
