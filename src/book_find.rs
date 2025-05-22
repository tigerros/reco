use crate::Opening;
use crate::book;
use alloc::collections::VecDeque;
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position};

#[expect(clippy::result_large_err, reason = "both are big")]
/// Finds the opening using [`book::ALL`] for the given game and initial position.
///
/// # Errors
/// A move is illegal.
pub fn find_from_moves(
    game: &[Move],
    initial_position: Chess,
) -> Result<Option<&'static Opening<'static, &'static str>>, PlayError<Chess>> {
    // Positions of the game, going in reverse
    let mut rev_setups = VecDeque::with_capacity(game.len());

    rev_setups.push_front(initial_position.to_setup(EnPassantMode::Legal));
    let mut current_position = initial_position;

    for r#move in game {
        current_position = current_position.play(*r#move)?;
        rev_setups.push_front(current_position.to_setup(EnPassantMode::Legal));
    }

    for setup in &rev_setups {
        for opening in book::ALL {
            if opening.setup.as_ref() == setup {
                return Ok(Some(opening));
            }
        }
    }

    Ok(None)
}
