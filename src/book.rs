pub use crate::book_gen::*;

use crate::Line;
use crate::book;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use shakmaty::Setup;
#[cfg(feature = "alloc")]
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position};

#[cfg(feature = "alloc")]
/// Like [`Variation::find_line_from_moves`](crate::Variation::find_line_from_moves), but
/// looks through [`book::ALL`].
///
/// # Errors
/// See [`Variation::find_line_from_moves`](crate::Variation::find_line_from_moves).
pub fn find_line_from_moves(
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
