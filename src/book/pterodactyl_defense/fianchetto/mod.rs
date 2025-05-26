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
/// Pterodactyl Defense: Fianchetto
pub static FIANCHETTO: Variation = Variation {
    name: "Fianchetto",
    parent: Some(&super::PTERODACTYL_DEFENSE),
    variations: &[
        &QUEEN_PTERANODON,
        &QUEEN_PTERODACTYL,
        &QUEEN_BENONI_PTERODACTYL,
        &KING_PTERODACTYL,
        &RHAMPHORHYNCHUS,
    ],
    lines: &[],
};
pub mod queen_pteranodon;
pub use queen_pteranodon::QUEEN_PTERANODON;
pub mod queen_pterodactyl;
pub use queen_pterodactyl::QUEEN_PTERODACTYL;
pub mod queen_benoni_pterodactyl;
pub use queen_benoni_pterodactyl::QUEEN_BENONI_PTERODACTYL;
pub mod king_pterodactyl;
pub use king_pterodactyl::KING_PTERODACTYL;
pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
