use crate::Code;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use shakmaty::{Move, Setup};

/// An entry in the opening book.
///
/// Fields are borrowed in order to be compatible with constants.
/// You can enable the `alloc` feature for an `OpeningOwned` struct.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Opening<'a, Variation>
where
    Variation: AsRef<str>,
{
    /// The ECO code of the opening.
    ///
    /// The C95 in `Ruy Lopez: Closed, Breyer`.
    pub code: Code,
    /// The top-level name of the opening.
    ///
    /// The `"Ruy Lopez"` in `Ruy Lopez: Closed, Breyer`.
    pub name: &'a str,
    /// The variation of the opening.
    /// Each item represents an extra layer of specificity.
    ///
    /// The `["Closed", "Breyer"]` in `Ruy Lopez: Closed, Breyer`.
    pub variation: &'a [Variation],
    /// The moves of this opening.
    pub moves: &'a [Move],
    /// The position that occurs after the last move in [`Self.moves`](Self#structfield.moves) is played.
    pub setup: &'a Setup,
}

#[cfg(feature = "alloc")]
/// Owned version of [`Opening`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpeningOwned {
    /// See [`Opening.code`](Opening#structfield.code).
    pub code: Code,
    /// See [`Opening.name`](Opening#structfield.name).
    pub name: String,
    /// See [`Opening.variation`](Opening#structfield.variation).
    pub variation: Vec<String>,
    /// See [`Opening.moves`](Opening#structfield.moves).
    pub moves: Vec<Move>,
    /// See [`Opening.setup`](Opening#structfield.setup).
    pub setup: Setup,
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a OpeningOwned> for Opening<'a, String> {
    /// Simply borrows each field of the [`OpeningOwned`].
    fn from(value: &'a OpeningOwned) -> Self {
        Self {
            code: value.code,
            name: &value.name,
            variation: &value.variation,
            moves: &value.moves,
            setup: &value.setup,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//
// }
