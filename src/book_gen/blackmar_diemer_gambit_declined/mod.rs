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
use deranged::RangedU8;
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
# use reco::book::BLACKMAR_DIEMER_GAMBIT_DECLINED;
assert_eq!(BLACKMAR_DIEMER_GAMBIT_DECLINED.original_name(), "Blackmar-Diemer Gambit Declined");
```"#
)]
pub static BLACKMAR_DIEMER_GAMBIT_DECLINED: Variation = Variation {
    name: "Blackmar-Diemer Gambit Declined",
    parent: None,
    variations: &[
        &BROMBACHER_COUNTERGAMBIT,
        &ELBERT_COUNTERGAMBIT,
        &GEDULT_DEFENSE,
        &LAMB_DEFENSE,
        &LANGEHEINICKE_DEFENSE,
        &O_KELLY_DEFENSE,
        &VIENNA_DEFENSE,
        &WEINSBACH_DEFENSE,
    ],
    lines: &[],
};
pub mod brombacher_countergambit;
pub use brombacher_countergambit::BROMBACHER_COUNTERGAMBIT;
pub mod elbert_countergambit;
pub use elbert_countergambit::ELBERT_COUNTERGAMBIT;
pub mod gedult_defense;
pub use gedult_defense::GEDULT_DEFENSE;
pub mod lamb_defense;
pub use lamb_defense::LAMB_DEFENSE;
pub mod langeheinicke_defense;
pub use langeheinicke_defense::LANGEHEINICKE_DEFENSE;
pub mod o_kelly_defense;
pub use o_kelly_defense::O_KELLY_DEFENSE;
pub mod vienna_defense;
pub use vienna_defense::VIENNA_DEFENSE;
pub mod weinsbach_defense;
pub use weinsbach_defense::WEINSBACH_DEFENSE;
