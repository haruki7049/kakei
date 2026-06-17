//! Currency module

#[derive(Debug, PartialEq, Clone)]
pub enum Currency {
    JPY(JPY),
    SATS(SATS),
}

impl Currency {
    pub fn jpy(inner: i64) -> Self {
        Self::JPY(JPY { inner })
    }

    pub fn sats(inner: i64) -> Self {
        Self::SATS(SATS { inner })
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JPY(i) => write!(f, "{}", i),
            Self::SATS(i) => write!(f, "{}", i),
        }
    }
}

// ----- Each Currency Unit -----

#[derive(Debug, PartialEq, Clone)]
pub struct JPY {
    inner: i64,
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

#[derive(Debug, PartialEq, Clone)]
pub struct SATS {
    inner: i64,
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
        let _ = Currency::jpy(1); // 1 JPY
        let _ = Currency::sats(1000); // 1000 SATS
        Ok(())
    }
}
