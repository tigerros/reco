use crate::{Code, Variation};
use core::fmt::Debug;
use shakmaty::{Move, Setup};

/// A line within a [`Variation`].
///
/// In order to improve performance, correctness, and usability (for most users), [`Variation`]
/// cannot be constructed outside of `reco`. See [`Variation`]s docs for more.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    pub(crate) parent: &'static Variation,
    pub(crate) code: Code,
    pub(crate) moves: &'static [Move],
    pub(crate) setup: Setup,
}

impl Line {
    /// The parent variation of this line.
    pub const fn parent(&'static self) -> &'static Variation {
        self.parent
    }

    /// The ECO code of the line.
    pub const fn code(&'static self) -> Code {
        self.code
    }

    /// The moves of the line.
    pub const fn moves(&'static self) -> &'static [Move] {
        self.moves
    }

    /// The position that occurs after the last move in [`Self.moves`](Self#structfield.moves) is played.
    pub const fn setup(&'static self) -> &'static Setup {
        &self.setup
    }
}

#[cfg(test)]
#[cfg(feature = "book")]
mod tests {
    use super::Line;

    /// Tests that the getters correspond to the fields.
    #[test]
    fn getters() {
        for variation in crate::book::ALL {
            variation.walk_with_self(&mut |variation| {
                for line in variation.lines() {
                    let Line {
                        parent,
                        code,
                        moves,
                        setup,
                    } = line;

                    assert_eq!(line.parent(), *parent);
                    assert_eq!(line.code(), *code);
                    assert_eq!(line.moves(), *moves);
                    assert_eq!(line.setup(), setup);
                }

                None::<()>
            });
        }
    }
}
