use crate::{Variation, Line};

/// A line within
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Opening {
    variation: &'static Variation,
    line: &'static Line,
}

impl Opening {
    pub const fn new(variati)
}