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
/// Neo-Grünfeld Defense
pub static NEO_GRUNFELD_DEFENSE: Variation = Variation {
    name: "Neo-Grünfeld Defense",
    parent: None,
    variations: &[
        &GOGLIDZE_ATTACK,
        &EXCHANGE_VARIATION,
        &DELAYED_EXCHANGE_VARIATION,
        &CLASSICAL_VARIATION,
        &ULTRA_DELAYED_EXCHANGE_VARIATION,
        &WITH_NF3,
        &WITH_G3,
    ],
    lines: &[],
};
pub mod goglidze_attack;
pub use goglidze_attack::GOGLIDZE_ATTACK;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod delayed_exchange_variation;
pub use delayed_exchange_variation::DELAYED_EXCHANGE_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod ultra_delayed_exchange_variation;
pub use ultra_delayed_exchange_variation::ULTRA_DELAYED_EXCHANGE_VARIATION;
pub mod with_nf3;
pub use with_nf3::WITH_NF3;
pub mod with_g3;
pub use with_g3::WITH_G3;
