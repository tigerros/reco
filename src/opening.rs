use crate::Code;
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use shakmaty::{Move, Setup};

/// An entry in the opening book.
///
/// Fields are borrowed in order to be compatible with constants.
/// You can enable the `alloc` feature for an `OpeningOwned` struct.
#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Opening<'a, Name = &'a str> {
    /// The ECO code of the opening.
    ///
    /// C95 for `Ruy Lopez: Closed, Breyer`.
    pub code: Code,
    /// The name of the opening.
    /// Each item represents an extra layer of specificity.
    ///
    /// `["Ruy Lopez", "Closed", "Breyer"]` for `Ruy Lopez: Closed, Breyer`.
    pub name: &'a [Name],
    /// The moves of this opening.
    pub moves: &'a [Move],
    /// The position that occurs after the last move in [`Self.moves`](Self#structfield.moves) is played.
    pub setup: &'a Setup,
}

impl<'a, Name, RhsName> PartialEq<Opening<'a, RhsName>> for Opening<'a, Name>
where
    Name: PartialEq<RhsName>,
{
    fn eq(&self, other: &Opening<'a, RhsName>) -> bool {
        self == other
    }
}

#[cfg(feature = "alloc")]
impl<'a, Name> From<Opening<'a, Name>> for OpeningOwned<Name>
where
    Name: Clone,
{
    /// Clones each field.
    fn from(value: Opening<'a, Name>) -> Self {
        OpeningOwned {
            code: value.code,
            name: value.name.to_vec(),
            moves: value.moves.to_vec(),
            setup: value.setup.to_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
/// Owned version of [`Opening`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpeningOwned<Name = String> {
    /// See [`Opening.code`](Opening#structfield.code).
    pub code: Code,
    /// See [`Opening.name`](Opening#structfield.name).
    pub name: Vec<Name>,
    /// See [`Opening.moves`](Opening#structfield.moves).
    pub moves: Vec<Move>,
    /// See [`Opening.setup`](Opening#structfield.setup).
    pub setup: Setup,
}

#[cfg(feature = "alloc")]
impl<'a, Name> From<&'a OpeningOwned<Name>> for Opening<'a, Name> {
    /// Borrows each field.
    fn from(value: &'a OpeningOwned<Name>) -> Self {
        Self {
            code: value.code,
            name: &value.name,
            moves: &value.moves,
            setup: &value.setup,
        }
    }
}
