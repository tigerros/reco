pub use crate::book_gen::*;

use crate::book;
use crate::{Line, Variation};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use shakmaty::Setup;
#[cfg(feature = "alloc")]
use shakmaty::{Chess, Move, PlayError};
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::sync::LazyLock;

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
    for setup in crate::generate_game_setups(initial_position, game)?
        .iter()
        .rev()
    {
        if let Some(found) = find_line_from_setup(setup) {
            return Ok(Some(found));
        }
    }

    Ok(None)
}

/// Like [`Variation::find_line_from_setup`](crate::Variation::find_line_from_setup), but
/// looks through [`book::ALL`].
///
/// Automatically uses [`SETUP_TO_LINE`] if the `std` feature is enabled.
pub fn find_line_from_setup(setup: &Setup) -> Option<&'static Line> {
    #[cfg(feature = "std")]
    {
        SETUP_TO_LINE.get(setup).copied()
    }

    #[cfg(not(feature = "std"))]
    {
        for variation in &book::ALL {
            if let Some(found) = variation.find_line_from_setup(setup) {
                return Some(found);
            }
        }

        None
    }
}

/// Maps every [`Setup`] to a [`Line`] to drastically improve lookup times.
#[cfg(feature = "std")]
pub static SETUP_TO_LINE: LazyLock<HashMap<&'static Setup, &'static Line>> = LazyLock::new(|| {
    // Assume every variation has 5 lines on average.
    // Shrunk at the end.
    let mut map = HashMap::with_capacity(book::VARIATION_COUNT * 5);

    book::walk_all_with_self(&mut |variation| {
        for line in variation.lines() {
            map.insert(line.setup(), line);
        }

        None::<()>
    });

    map.shrink_to_fit();

    map
});

/// Contains every single [`Variation`] in the [`book::ALL`] tree.
#[cfg(feature = "book-flattened")]
pub static FLATTENED: [&Variation; VARIATION_COUNT] = {
    const fn add_variation(
        flattened: &mut [&Variation; VARIATION_COUNT],
        index: &mut usize,
        variation: &'static Variation,
    ) {
        #[expect(clippy::indexing_slicing, reason = "build will fail if this panics")]
        {
            flattened[*index] = variation;
        }
        *index += 1;

        // Add all child variations recursively
        let mut i = 0;
        while i < variation.variations.len() {
            #[expect(clippy::indexing_slicing, reason = "build will fail if this panics")]
            add_variation(flattened, index, variation.variations[i]);
            i += 1;
        }
    }

    let mut flattened: [&Variation; VARIATION_COUNT] = [&Variation {
        name: "",
        parent: None,
        variations: &[],
        lines: &[],
    }; VARIATION_COUNT];

    let mut index = 0;

    let mut root_index = 0;
    while root_index < book::ALL.len() {
        #[expect(clippy::indexing_slicing, reason = "build will fail if this panics")]
        add_variation(&mut flattened, &mut index, book::ALL[root_index]);
        root_index += 1;
    }

    flattened
};

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

    /// Tests that the contents of [`SETUP_TO_LINE`] are correct.
    #[cfg(feature = "std")]
    #[test]
    fn setup_to_line() {
        book::walk_all_with_self(&mut |variation| {
            for line in variation.lines() {
                assert_eq!(SETUP_TO_LINE.get(line.setup()).copied(), Some(line));
            }

            None::<()>
        });
    }

    /// Tests that the contents of [`FLATTENED`] are correct.
    #[cfg(feature = "book-flattened")]
    #[test]
    fn flattened() {
        let mut flattened = alloc::vec::Vec::new();

        book::walk_all_with_self(&mut |variation| {
            flattened.push(variation);

            None::<()>
        });

        assert_eq!(FLATTENED.as_slice(), flattened);
    }
}
