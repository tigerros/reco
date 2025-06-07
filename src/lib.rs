#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::arithmetic_side_effects,
    clippy::unchecked_duration_subtraction,
    clippy::as_conversions,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::modulo_one,
    clippy::iterator_step_by_zero,
    clippy::invalid_regex,
    clippy::print_stdout,
    clippy::print_stderr,
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    clippy::allow_attributes,
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::cfg_not_test,
    clippy::clone_on_ref_ptr,
    clippy::string_add,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::tests_outside_test_module
)]
#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exit,
        clippy::todo,
        clippy::unreachable,
        clippy::panic_in_result_fn,
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::get_unwrap
    )
)]
#![allow(
    clippy::must_use_candidate,
    reason = "triggers too much, kind of pointless"
)]
#![allow(
    clippy::unreadable_literal,
    reason = "triggers only on the generated bitboards"
)]
// build docs.rs-like documentation locally with cargo +nightly doc --all-features --no-deps --open
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

pub mod code;
mod line;
pub mod volume;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
pub use code::Code;
pub use line::Line;
#[cfg(feature = "alloc")]
use shakmaty::{Chess, EnPassantMode, Move, PlayError, Position, Setup};
pub use volume::Volume;
#[cfg(feature = "book")]
#[cfg_attr(docsrs, doc(cfg(feature = "book")))]
mod book_gen;

#[cfg(feature = "book")]
#[cfg_attr(docsrs, doc(cfg(feature = "book")))]
pub mod book;
mod variation;
pub use variation::{ValidityError, Variation};

/// The 0-99 category of an opening.
pub type Category = deranged::RangedU8<0, 99>;

#[cfg(feature = "alloc")]
/// Generates a setup for each point in the given game.
/// The first setup is the `initial_position`.
///
/// This is useful if you want to find a [`Line`] based on a list of moves.
/// Iterate over the setups in reverse, look for the line where the setup matches, and
/// that's the closest matching line.
///
/// # Errors
/// A move is illegal.
pub fn generate_game_setups(
    initial_position: Chess,
    game: &[Move],
) -> Result<Vec<Setup>, Box<PlayError<Chess>>> {
    #[expect(clippy::arithmetic_side_effects, reason = "it won't ever be this big")]
    let mut setups = Vec::with_capacity(game.len() + 1);

    setups.push(initial_position.to_setup(EnPassantMode::Legal));
    let mut current_position = initial_position;

    for r#move in game {
        current_position = current_position.play(*r#move).map_err(Box::new)?;
        setups.push(current_position.to_setup(EnPassantMode::Legal));
    }

    Ok(setups)
}
