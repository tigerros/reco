use crate::book;
use crate::{Line, Variation};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position, Setup};

#[cfg(feature = "alloc")]
/// Finds the variation and the line using [`book::ALL`] for the given game and initial position.
///
/// # Errors
/// A move is illegal.
pub fn find_from_moves(
    game: &[Move],
    initial_position: Chess,
) -> Result<Option<(&'static Variation, &'static Line)>, Box<PlayError<Chess>>> {
    // Positions of the game, going in reverse
    let mut rev_setups = VecDeque::with_capacity(game.len());

    rev_setups.push_front(initial_position.to_setup(EnPassantMode::Legal));
    let mut current_position = initial_position;

    for r#move in game {
        current_position = current_position.play(*r#move).map_err(Box::new)?;
        rev_setups.push_front(current_position.to_setup(EnPassantMode::Legal));
    }

    for setup in rev_setups {
        if let Some(found) = find_from_setup(setup) {
            return Ok(Some(found));
        }
    }

    Ok(None)
}

pub fn find_from_setup(setup: Setup) -> Option<(&'static Variation, &'static Line)> {
    for variation in &book::ALL {
        let result = variation.walk_with_self(&mut |subvariation| {
            for line in subvariation.lines() {
                if line.setup == setup {
                    return Some((subvariation, line));
                }
            }

            None
        });

        if result.is_some() {
            return result;
        }
    }

    None
}