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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
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
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::BENKO_GAMBIT_DECLINED;
assert_eq!(BENKO_GAMBIT_DECLINED.original_name(), "Benko Gambit Declined");
```"#
)]
pub static BENKO_GAMBIT_DECLINED: Variation = Variation {
    name: "Benko Gambit Declined",
    parent: None,
    variations: &[
        &BISHOP_ATTACK,
        &HJORRING_COUNTERGAMBIT,
        &MAIN_LINE,
        &PSEUDO_SAMISCH,
        &QUIET_LINE,
        &SOSONKO_VARIATION,
    ],
    lines: &[],
};
pub mod bishop_attack;
pub use bishop_attack::BISHOP_ATTACK;
pub mod hjorring_countergambit;
pub use hjorring_countergambit::HJORRING_COUNTERGAMBIT;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod pseudo_samisch;
pub use pseudo_samisch::PSEUDO_SAMISCH;
pub mod quiet_line;
pub use quiet_line::QUIET_LINE;
pub mod sosonko_variation;
pub use sosonko_variation::SOSONKO_VARIATION;
