use crate::Line;
#[cfg(feature = "alloc")]
use crate::generate_game_setups;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use shakmaty::Setup;
#[cfg(feature = "alloc")]
use shakmaty::{Chess, Move, PlayError};

#[expect(missing_copy_implementations, reason = "too big")]
/// A named variation.
///
/// In order to improve performance, correctness, and usability (for most users), [`Variation`]
/// cannot be constructed outside of `reco`.
///
/// The only situation I can think of where this might
/// be a problem, is if you want to use your own source instead of [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
/// However, I'm not aware of any better source, and if your source contains data not present here,
/// you wouldn't be able to use this struct anyway.
#[derive(Debug, Clone)]
pub struct Variation {
    pub(crate) name: &'static str,
    pub(crate) parent: Option<&'static Self>,
    pub(crate) variations: &'static [&'static Self],
    pub(crate) lines: &'static [Line],
}

impl PartialEq for Variation {
    /// Uses pointer equality, that is, uses [`core::ptr::eq`].
    ///
    /// This is necessary due to the fact that comparing by value will overflow the stack, because
    /// of the tree structure.
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self, other)
    }
}

impl Eq for Variation {}

impl Hash for Variation {
    /// Hashes are generated from pointers due to using pointer equality in
    /// <a href="#impl-PartialEq-for-Variation">`impl PartialEq for Variation`</a>.
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::ptr::hash(self, state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// See [`Variation::validity`].
pub enum ValidityError {
    /// `self` is not a root variation (it has a parent).
    NonRoot,
    /// A variation has a parent, but an incorrect one.
    IncorrectParent(&'static Variation),
    /// A variation does not have a parent.
    MissingParent(&'static Variation),
}

impl Variation {
    /// The name of this variation.
    ///
    /// This is not the full name, you'll need to look at all the parents to get that.
    pub const fn name(&'static self) -> &'static str {
        self.name
    }

    /// The parent variation of this variation.
    ///
    /// [`None`] if this is a root variation.
    pub const fn parent(&'static self) -> Option<&'static Self> {
        self.parent
    }

    /// The variations of this variation.
    pub const fn variations(&'static self) -> &'static [&'static Self] {
        self.variations
    }

    /// A variation with the same name can contain multiple lines.
    ///
    /// For example, A05 Zukertort has two lines:
    /// - `1. Nf3 Nf6`
    /// - `1. Nf3 Nf6 2. Nc3 Nc6`
    pub const fn lines(&'static self) -> &'static [Line] {
        self.lines
    }

    /// Finds the root of this variation.
    ///
    /// Returns `self` if `self` is a root variation.
    #[cfg_attr(
        feature = "book",
        doc = r"
# Examples
```rust
use reco::book::SICILIAN_DEFENSE;
use reco::book::sicilian_defense::accelerated_dragon::maroczy_bind::BREYER_VARIATION;

assert_eq!(BREYER_VARIATION.root(), &SICILIAN_DEFENSE);
assert_eq!(SICILIAN_DEFENSE.root(), &SICILIAN_DEFENSE);
```"
    )]
    pub const fn root(&'static self) -> &'static Self {
        let Some(mut parent) = self.parent else {
            return self;
        };

        while let Some(grandparent) = parent.parent {
            parent = grandparent;
        }

        parent
    }

    /// Recursively walks [`Self::variations`], stopping if the
    /// `walker` returns [`Some`], returning that value.
    ///
    /// First, it walks a variation and then it's subvariations.
    ///
    /// See also [`Self::walk_with_self`].
    pub fn walk<F, T>(&'static self, walker: &mut F) -> Option<T>
    where
        F: FnMut(&'static Self) -> Option<T>,
    {
        for variation in self.variations {
            if let Some(value) = variation.walk_with_self(walker) {
                return Some(value);
            }
        }

        None
    }

    /// Like [`Self::walk`], but also walks `self` initially.
    pub fn walk_with_self<F, T>(&'static self, walker: &mut F) -> Option<T>
    where
        F: FnMut(&'static Self) -> Option<T>,
    {
        if let Some(t) = walker(self) {
            return Some(t);
        }

        for variation in self.variations {
            if let Some(t) = variation.walk_with_self(walker) {
                return Some(t);
            }
        }

        None
    }

    /// Like [`Self::validity`], but doesn't error on a non-root variation.
    ///
    /// [`ValidityError::NonRoot`] is guaranteed not to occur.
    fn non_root_validity(&'static self) -> Result<(), ValidityError> {
        for variation in self.variations {
            let Some(parent) = variation.parent else {
                return Err(ValidityError::MissingParent(variation));
            };

            if !core::ptr::eq(parent, self) {
                return Err(ValidityError::IncorrectParent(variation));
            }

            variation.non_root_validity()?;
        }

        Ok(())
    }

    /// Recursively checks if [`Self::variations`] all have `self` as the parent.
    /// Can only be called on a root variation.
    ///
    /// # Errors
    /// See [`ValidityError`].
    pub fn validity(&'static self) -> Result<(), ValidityError> {
        if self.parent.is_some() {
            return Err(ValidityError::NonRoot);
        }

        self.non_root_validity()?;

        Ok(())
    }

    /// Uses [`Self::walk_with_self`] to find the line that matches the given setup.
    pub fn find_line_from_setup(&'static self, setup: &Setup) -> Option<&'static Line> {
        self.walk_with_self(&mut |subvariation| {
            subvariation
                .lines()
                .iter()
                .find(|line| &line.setup == setup)
        })
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Uses [`Self::walk_with_self`] to find a line that most closely matches the given "game".
    /// That is, an initial position and a list of moves.
    ///
    /// Starts by looking for the final position of the game, going backwards until the initial
    /// position. For example, let's say we have the game `[e4, e5, d4]`. This function will:
    /// - Look for the position obtained by playing `[e4, e5, d4]`.
    /// - Look for the position obtained by playing `[e4, e5]`.
    /// - Look for the position obtained by playing `[e4]`.
    /// - Look for `initial_position`.
    ///
    /// This ensures that the most specific line is found.
    ///
    /// # Errors
    /// A move is illegal.
    pub fn find_line_from_moves(
        &'static self,
        initial_position: Chess,
        moves: &[Move],
    ) -> Result<Option<&'static Line>, Box<PlayError<Chess>>> {
        // Reverse them otherwise it just finds the root
        for setup in generate_game_setups(initial_position, moves)?.iter().rev() {
            if let Some(found) = self.find_line_from_setup(setup) {
                return Ok(Some(found));
            }
        }

        Ok(None)
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Gets the original name, as seen in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
    ///
    /// For example, `"Sicilian Defense: Najdorf Variation, Main Line"`.
    ///
    /// If `self` is a root variation, [`Self::name`] is returned and nothing is allocated.
    pub fn original_name(&'static self) -> Cow<'static, str> {
        let Some(mut parent) = self.parent else {
            return Cow::Borrowed(self.name);
        };

        let mut names = Vec::with_capacity(10);

        names.push(self.name);
        names.push(parent.name);

        while let Some(grandparent) = parent.parent {
            names.push(grandparent.name);
            parent = grandparent;
        }

        let mut name = String::with_capacity(
            self.name
                .len()
                .saturating_add(parent.name.len())
                .saturating_mul(3),
        );

        #[expect(
            clippy::indexing_slicing,
            clippy::arithmetic_side_effects,
            reason = "names is guaranteed to have self.name and parent.name"
        )]
        name.push_str(names[names.len() - 1]);
        name.push_str(": ");

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "names is guaranteed to have self.name and parent.name"
        )]
        let mut i = names.len() - 2;

        loop {
            #[expect(clippy::indexing_slicing, reason = "i is within bounds")]
            name.push_str(names[i]);

            if i == 0 {
                break;
            }

            name.push_str(", ");

            #[expect(
                clippy::arithmetic_side_effects,
                reason = "we check if i is 0 then break"
            )]
            {
                i -= 1;
            }
        }

        Cow::Owned(name)
    }
}

#[cfg(test)]
#[cfg(feature = "book")]
#[cfg_attr(
    feature = "alloc",
    expect(clippy::unwrap_used, clippy::expect_used, reason = "tests")
)]
mod tests {
    use super::Variation;
    use crate::book;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;
    use std::collections::HashSet;

    /// Tests that the getters correspond to the fields.
    #[test]
    fn getters() {
        for variation in book::ALL {
            variation.walk_with_self(&mut |variation| {
                let Variation {
                    name,
                    parent,
                    variations,
                    lines,
                } = variation;

                assert_eq!(variation.name(), *name);
                assert_eq!(variation.parent(), *parent);
                assert_eq!(variation.variations(), *variations);
                assert_eq!(variation.lines(), *lines);

                None::<()>
            });
        }
    }

    /// A double test; tests that [`Variation::root`] is correct, using [`Variation::walk_with_self`].
    #[test]
    fn root() {
        for root in book::ALL {
            root.walk_with_self(&mut |variation| {
                assert_eq!(variation.root(), root);

                None::<()>
            });
        }
    }

    /// A double test; tests that [`Variation::walk`] used on [`book::ALL`] records the same number
    /// of variations as [`book::VARIATION_COUNT`] minus `book::ALL.len()`.
    #[test]
    fn walk() {
        // Use HashSet to make sure the walkers don't walk duplicates.
        let mut walked = HashSet::new();

        for root in book::ALL {
            root.walk(&mut |variation| {
                walked.insert(variation);

                None::<()>
            });
        }

        assert_eq!(walked.len(), book::VARIATION_COUNT - book::ALL.len());
    }

    /// A double test; tests that [`Variation::walk_with_self`] used on [`book::ALL`]
    /// records the same number of variations as [`book::VARIATION_COUNT`].
    #[test]
    fn walk_with_self() {
        let mut walked = HashSet::new();

        for root in book::ALL {
            root.walk_with_self(&mut |variation| {
                walked.insert(variation);

                None::<()>
            });
        }

        assert_eq!(walked.len(), book::VARIATION_COUNT);
    }

    /// A double test; tests that [`Variation::walk`] short-circuits when the `walker` function returns [`Some`]
    /// and that [`book::walk_all`] works as expected.
    #[test]
    fn walk_short_circuit() {
        const MAX_VARIATIONS: usize = 1123;
        let mut walked = HashSet::new();

        book::walk_all(&mut |variation| {
            walked.insert(variation);

            if walked.len() == MAX_VARIATIONS {
                return Some(walked.len());
            }

            None
        });

        assert_eq!(walked.len(), MAX_VARIATIONS);

        for walked in walked {
            // Make sure the book doesn't contain walked, because we didn't use `_with_self`.
            assert!(!book::ALL.contains(&walked));
        }
    }

    /// Tests that [`Variation::walk_with_self`] short-circuits when the `walker` function returns [`Some`].
    #[test]
    fn walk_with_self_short_circuit() {
        const MAX_VARIATIONS: usize = 1123;
        let mut walked = HashSet::new();

        book::walk_all_with_self(&mut |variation| {
            walked.insert(variation);

            if walked.len() == MAX_VARIATIONS {
                return Some(walked.len());
            }

            None
        });

        assert_eq!(walked.len(), MAX_VARIATIONS);
    }

    /// Tests that [`Variation::find_line_from_setup`] and [`book::find_line_from_setup`]
    /// find the correct line, using [`book::ALL`].
    #[test]
    fn find_line_from_setup() {
        for root in book::ALL {
            root.walk_with_self(&mut |variation| {
                for line in variation.lines() {
                    assert_eq!(variation.find_line_from_setup(&line.setup), Some(line));
                    assert_eq!(book::find_line_from_setup(&line.setup), Some(line));
                }

                None::<()>
            });
        }
    }

    /// Tests that [`Variation::find_line_from_moves`] and [`book::find_line_from_moves`]
    /// find the correct line, using [`book::ALL`].
    ///
    /// Since [`Variation::find_line_from_moves`] accept both an initial position and a move history,
    /// each combination is tested.
    ///
    /// For example, if we check the line `1. e4 e5 2. d4`, we test that
    /// [`Variation::find_line_from_moves`] finds the correct line in these cases:
    /// - `position = @[], moves = [e4, e5, d4]`
    /// - `position = @[e4], moves = [e5, d4]`
    /// - `position = @[e4, e5], moves = [d4]`
    /// - `position = @[e4, e5, d4], moves = []`
    ///
    /// Where `@[m1, m2]` is just the position of the given continuation.
    #[cfg(feature = "alloc")]
    #[test]
    fn find_line_from_moves() {
        use shakmaty::{Chess, Position};

        for root in book::ALL {
            root.walk_with_self(&mut |variation| {
                for line in variation.lines() {
                    // We need to build a history of positions and histories of moves.
                    // We make "games", where each game is a tuple of a position and a move history,
                    // where each position gets the necessary move history to get to the line position.
                    //
                    // For example, the initial position gets the full move history.
                    // The final position (that is, the line position) doesn't get any moves.
                    let mut games = Vec::with_capacity(line.moves().len() + 1);
                    let mut current_move_history = line.moves().to_vec();
                    let mut current_position = Chess::new();

                    // We start out with the initial position and then the full move history.
                    games.push((current_position.clone(), current_move_history.clone()));

                    // For each move, we play it on the position, and remove the first move from the move history.
                    for &r#move in line.moves() {
                        current_position =
                            current_position.play(r#move).expect("move should be legal");
                        current_move_history.remove(0);

                        games.push((current_position.clone(), current_move_history.clone()));
                    }

                    for (position, move_history) in games {
                        assert_eq!(
                            variation
                                .find_line_from_moves(position.clone(), &move_history)
                                .expect("move_history should be legal"),
                            Some(line)
                        );

                        assert_eq!(
                            book::find_line_from_moves(position.clone(), &move_history)
                                .expect("move_history should be legal"),
                            Some(line)
                        );
                    }
                }

                None::<()>
            });
        }
    }

    /// Tests that [`Variation::find_line_from_moves`] and [`book::find_line_from_moves`]
    /// return `Ok(None)` when appropriate.
    #[cfg(feature = "alloc")]
    #[test]
    fn dont_find_line_from_moves() {
        use shakmaty::{Chess, fen::Fen};

        // No opening
        let position: Chess =
            Fen::from_ascii(b"rnbqkbnr/pp1ppppp/8/2p5/P7/N7/1PPPPPPP/R1BQKBNR b KQkq - 1 2")
                .unwrap()
                .into_position(shakmaty::CastlingMode::Standard)
                .unwrap();

        for root in book::ALL {
            root.walk_with_self(&mut |variation| {
                assert_eq!(
                    variation
                        .find_line_from_moves(position.clone(), &[])
                        .expect("moves are empty"),
                    None
                );

                assert_eq!(
                    book::find_line_from_moves(position.clone(), &[]).expect("moves are empty"),
                    None
                );

                None::<()>
            });
        }
    }
}
