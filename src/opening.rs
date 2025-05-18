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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Volume;
    use alloc::string::{String, ToString};
    use alloc::{format, vec};
    use deranged::RangedU8;
    use shakmaty::Square;

    /// Returns the same opening but with different name types and mixed [`Cow`] variants.
    fn opening() -> (
        Opening<'static, String>,
        Opening<'static, Cow<'static, str>>,
    ) {
        const SETUP: &Setup = &Setup::initial();

        (
            Opening {
                code: Code {
                    volume: Volume::A,
                    category: RangedU8::new_static::<67>(),
                },
                name: Cow::Owned(vec!["Grünfeld".to_string(), "Smyslov".to_string()]),
                moves: Cow::Borrowed(&[Move::Castle {
                    king: Square::A2,
                    rook: Square::H8,
                }]),
                setup: Cow::Owned(Setup::initial()),
            },
            Opening {
                code: Code {
                    volume: Volume::A,
                    category: RangedU8::new_static::<67>(),
                },
                name: Cow::Borrowed(&[Cow::Borrowed("Grünfeld"), Cow::Borrowed("Smyslov")]),
                moves: Cow::Owned(vec![Move::Castle {
                    king: Square::A2,
                    rook: Square::H8,
                }]),
                setup: Cow::<'static, Setup>::Borrowed(SETUP),
            },
        )
    }

    #[test]
    fn eq() {
        assert_eq!(opening().0, opening().1);
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:#?}", opening().0),
            r#"Opening {
    code: Code {
        volume: A,
        category: 67,
    },
    name: [
        "Grünfeld",
        "Smyslov",
    ],
    moves: [
        Castle {
            king: A2,
            rook: H8,
        },
    ],
    setup: Setup {
        board: r n b q k b n r
        p p p p p p p p
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        P P P P P P P P
        R N B Q K B N R
        ,
        promoted: . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        ,
        pockets: None,
        turn: White,
        castling_rights: 1 . . . . . . 1
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        . . . . . . . .
        1 . . . . . . 1
        ,
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: 1,
    },
}"#
        );
    }
}
