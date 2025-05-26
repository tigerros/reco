use crate::{Line, Variation};
use crate::book;
use alloc::collections::VecDeque;
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position};
use alloc::boxed::Box;

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
        for variation in &book::ALL {
            variation.walk_with_self(|subvariation| {
                for line in subvariation.lines {
                    if line.setup == setup {
                        return Some((subvariation, line));
                    }
                }
                
                None
            });
        }
    }

    Ok(None)
}
