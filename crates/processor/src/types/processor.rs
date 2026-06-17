//! Processor

use crate::prelude::*;
use thiserror::Error;

/// Processor trait to express the processor methods
pub trait Processor<N>
where
    N: Note,
{
    type Error;

    /// Set a `Note` to render a table.
    fn set(&mut self, note: N);
}

#[derive(Debug)]
pub struct KakeiProcessor<N>
where
    N: Note,
{
    pub note: N,
}

#[derive(Debug, Error)]
pub enum KakeiProcessorError {}

impl<N> Processor<N> for KakeiProcessor<N>
where
    N: Note,
{
    type Error = KakeiProcessorError;

    fn set(&mut self, note: N) {
        self.note = note;
    }
}
