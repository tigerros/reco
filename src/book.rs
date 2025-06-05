pub use crate::book_gen::*;

use crate::{Line, Variation};
use crate::book;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use shakmaty::Setup;
#[cfg(feature = "alloc")]
use shakmaty::{Chess, Move, PlayError};

/// Uses [`Variation::walk`] on each of the [`book::ALL`] root variations.
/// Does not walk root variations themselves.
///
/// See also [`walk_all_with_self`].
pub fn walk_all<F, T>(walker: &mut F) -> Option<T>
where
    F: FnMut(&'static Variation) -> Option<T>,
{
    for root in book::ALL {
        if let Some(t) = root.walk(walker) {
            return Some(t);
        }
    }

    None
}

/// Like [`walk_all`], but also walks the root variations of [`book::ALL`].
pub fn walk_all_with_self<F, T>(walker: &mut F) -> Option<T>
where
    F: FnMut(&'static Variation) -> Option<T>,
{
    for root in book::ALL {
        if let Some(t) = root.walk_with_self(walker) {
            return Some(t);
        }
    }

    None
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
/// Like [`Variation::find_line_from_moves`](crate::Variation::find_line_from_moves), but
/// looks through [`book::ALL`].
///
/// # Errors
/// See [`Variation::find_line_from_moves`](crate::Variation::find_line_from_moves).
pub fn find_line_from_moves(
    initial_position: Chess,
    game: &[Move],
) -> Result<Option<&'static Line>, Box<PlayError<Chess>>> {
    // Reverse them otherwise it just finds the root
    for setup in crate::generate_game_setups(initial_position, game)?.iter().rev() {
        if let Some(found) = find_line_from_setup(setup) {
            return Ok(Some(found));
        }
    }

    Ok(None)
}

/// Like [`Variation::find_line_from_setup`](crate::Variation::find_line_from_setup), but
/// looks through [`book::ALL`].
pub fn find_line_from_setup(setup: &Setup) -> Option<&'static Line> {
    for variation in &book::ALL {
        if let Some(found) = variation.find_line_from_setup(setup) {
            return Some(found);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_all() {
        let mut variation_count = 0;

        book::walk_all(&mut |_| {
            variation_count += 1;

            None::<()>
        });

        assert_eq!(variation_count, book::VARIATION_COUNT - book::ALL.len());
    }

    #[test]
    fn walk_all_with_self() {
        let mut variation_count = 0;

        book::walk_all_with_self(&mut |_| {
            variation_count += 1;

            None::<()>
        });

        assert_eq!(variation_count, book::VARIATION_COUNT);
    }
}
