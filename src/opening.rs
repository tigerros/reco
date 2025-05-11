use crate::Code;
use shakmaty::{Move, Setup};

/// A full entry in the opening book.
///
/// The generics are necessary to be compatible with `String` and `&str`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Opening<'a, Name, Variation>
where
    Name: AsRef<str>,
    Variation: AsRef<str>,
{
    pub code: Code,
    /// The top-level name of the opening.
    pub name: Name,
    /// The variation of the opening.
    ///
    /// This is an array because there may be multiple variation layers.
    pub variation: &'a [Variation],
    pub moves: &'a [Move],
    pub setup: &'a Setup,
}
