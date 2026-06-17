//! Processor

/// Processor trait to express the processor methods
pub trait Processor {}

#[derive(Debug)]
pub struct KakeiProcessor {}

impl Processor for KakeiProcessor {}
