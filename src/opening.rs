use crate::Code;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use core::fmt::Debug;
use shakmaty::{Move, Setup};

/// An entry in the opening book.
///
/// Lists are [`Cow`]s for the struct to be const-available.
#[derive(Clone, Eq, Hash)]
pub struct Opening<'a, Name>
where
    [Name]: ToOwned,
{
    /// The ECO code of the opening.
    pub code: Code,
    /// The name of the opening.
    /// Each item represents an extra layer of specificity.
    ///
    /// For example, `["Ruy Lopez", "Closed", "Breyer"]`.
    ///
    /// Generic to allow for various string types.
    pub name: Cow<'a, [Name]>,
    /// The moves of this opening.
    pub moves: Cow<'a, [Move]>,
    /// The position that occurs after the last move in [`Self.moves`](Self#structfield.moves) is played.
    pub setup: Cow<'a, Setup>,
}

impl<Name, RhsName> PartialEq<Opening<'_, RhsName>> for Opening<'_, Name>
where
    [Name]: ToOwned,
    [RhsName]: ToOwned,
    Name: PartialEq<RhsName>,
{
    fn eq(&self, other: &Opening<'_, RhsName>) -> bool {
        self.code == other.code
            && self.name == other.name
            && self.moves == other.moves
            && self.setup == other.setup
    }
}

impl<Name> Debug for Opening<'_, Name>
where
    [Name]: ToOwned,
    <[Name] as ToOwned>::Owned: Debug,
    Name: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Opening")
            .field("code", &self.code)
            .field("name", &self.name)
            .field("moves", &self.moves)
            .field("setup", &self.setup)
            .finish()
    }
}
