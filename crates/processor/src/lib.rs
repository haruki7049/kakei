// ----- Processor -----

/// Processor trait to express the processor methods
pub trait Processor {}

#[derive(Debug)]
pub struct KakeiProcessor {}

impl Processor for KakeiProcessor {}

// ----- Kakeibo -----

pub trait Kakeibo {}

#[derive(Debug)]
pub struct KakeiboNote {}

impl Kakeibo for KakeiboNote {}

// ----- Configuration -----

#[derive(Debug)]
pub struct Configuration {}
