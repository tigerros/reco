use crate::Code;
use shakmaty::{Move, Setup};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Opening<'a> {
    pub code: Code,
    /// The top-level name of the opening.
    pub name: &'a str,
    /// The variation of the opening.
    ///
    /// This is an array because there may be multiple variation layers.
    pub variation: &'a [&'a str],
    pub moves: &'a [Move],
    pub setup: &'a Setup,
}
