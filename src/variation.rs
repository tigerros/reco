use crate::Line;
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
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position};

#[expect(
    missing_copy_implementations,
    reason = "copy semantics conflict with the tree structure"
)]
/// A named variation.
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
    ///
    /// # Examples
    /// ```rust
    /// use reco::book::SICILIAN_DEFENSE;
    /// use reco::book::sicilian_defense::accelerated_dragon::maroczy_bind::BREYER_VARIATION;
    ///
    /// assert_eq!(BREYER_VARIATION.root(), &SICILIAN_DEFENSE);
    /// assert_eq!(SICILIAN_DEFENSE.root(), &SICILIAN_DEFENSE);
    /// ```
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
    /// `walker` returns [`Some`], while returning that value.
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

    /// Uses [`Self::walk_with_self`] to find the line that matches the given setup.
    pub fn find_line_from_setup(&'static self, setup: &Setup) -> Option<&'static Line> {
        self.walk_with_self(&mut |subvariation| {
            subvariation
                .lines()
                .iter()
                .find(|&line| &line.setup == setup)
        })
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

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Uses [`Self::walk_with_self`] to find a line that matches the given "game".
    ///
    /// That is, the given list of moves played on the given initial position.
    ///
    /// # Errors
    /// A move is illegal.
    pub fn find_line_from_moves(
        &'static self,
        game: &[Move],
        initial_position: Chess,
    ) -> Result<Option<&'static Line>, Box<PlayError<Chess>>> {
        // Positions of the game
        let mut setups = Vec::with_capacity(game.len());

        setups.push(initial_position.to_setup(EnPassantMode::Legal));
        let mut current_position = initial_position;

        for r#move in game {
            current_position = current_position.play(*r#move).map_err(Box::new)?;
            setups.push(current_position.to_setup(EnPassantMode::Legal));
        }

        // Reverse them otherwise it just finds the root
        for setup in setups.iter().rev() {
            if let Some(found) = self.find_line_from_setup(setup) {
                return Ok(Some(found));
            }
        }

        Ok(None)
    }

    // TODO: Tests seem alarmingly slow.
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
mod tests {
    use super::*;
    #[cfg(feature = "book")]
    use crate::book;

    /// Tests that the getters correspond to the fields.
    #[test]
    #[cfg(feature = "book")]
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
    #[cfg(feature = "book")]
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
    #[cfg(feature = "book")]
    fn walk() {
        let mut variation_count = 0;

        for root in book::ALL {
            root.walk(&mut |_| {
                variation_count += 1;

                None::<()>
            });
        }

        assert_eq!(book::VARIATION_COUNT - book::ALL.len(), variation_count);
    }

    /// A double test; tests that [`Variation::walk_with_self`] used on [`book::ALL`]
    /// records the same number of variations as [`book::VARIATION_COUNT`].
    #[test]
    #[cfg(feature = "book")]
    fn walk_with_self() {
        let mut variation_count = 0;

        for root in book::ALL {
            root.walk_with_self(&mut |_| {
                variation_count += 1;

                None::<()>
            });
        }

        assert_eq!(book::VARIATION_COUNT, variation_count);
    }
}
