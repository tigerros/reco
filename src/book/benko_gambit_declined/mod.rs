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
/// Benko Gambit Declined
pub static BENKO_GAMBIT_DECLINED: Variation = Variation {
    name: "Benko Gambit Declined",
    parent: None,
    variations: &[
        &SOSONKO_VARIATION,
        &BISHOP_ATTACK,
        &PSEUDO_SAMISCH,
        &MAIN_LINE,
        &HJORRING_COUNTERGAMBIT,
        &QUIET_LINE,
    ],
    lines: &[],
};
pub mod sosonko_variation;
pub use sosonko_variation::SOSONKO_VARIATION;
pub mod bishop_attack;
pub use bishop_attack::BISHOP_ATTACK;
pub mod pseudo_samisch;
pub use pseudo_samisch::PSEUDO_SAMISCH;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod hjorring_countergambit;
pub use hjorring_countergambit::HJORRING_COUNTERGAMBIT;
pub mod quiet_line;
pub use quiet_line::QUIET_LINE;
