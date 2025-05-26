#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Color::{Black, White};
#[allow(
    unused_imports,
    clippy::enum_glob_use,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Move::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};
#[allow(
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Queen's Gambit Accepted: Central Variation
pub static CENTRAL_VARIATION: Variation = Variation {
    name: "Central Variation",
    parent: Some(&super::QUEENS_GAMBIT_ACCEPTED),
    variations: &[
        &RUBINSTEIN_DEFENSE,
        &GRECO_VARIATION,
        &ALEKHINE_SYSTEM,
        &MC_DONNELL_DEFENSE,
        &MODERN_DEFENSE,
    ],
    lines: &[],
};
pub mod rubinstein_defense;
pub use rubinstein_defense::RUBINSTEIN_DEFENSE;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
pub mod alekhine_system;
pub use alekhine_system::ALEKHINE_SYSTEM;
pub mod mc_donnell_defense;
pub use mc_donnell_defense::MC_DONNELL_DEFENSE;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
