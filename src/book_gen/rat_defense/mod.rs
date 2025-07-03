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
use core::unreachable;
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
# use reco::book::RAT_DEFENSE;
assert_eq!(RAT_DEFENSE.original_name(), "Rat Defense");
```"#
)]
pub static RAT_DEFENSE: Variation = Variation {
    name: "Rat Defense",
    parent: None,
    variations: &[
        &ACCELERATED_GURGENIDZE,
        &ANTAL_DEFENSE,
        &BALOGH_DEFENSE,
        &ENGLISH_RAT,
        &FULLER_GAMBIT,
        &HARMONIST,
        &PETRUCCIOLI_ATTACK,
        &SMALL_CENTER_DEFENSE,
        &SPIKE_ATTACK,
    ],
    lines: &[],
};
pub mod accelerated_gurgenidze;
pub use accelerated_gurgenidze::ACCELERATED_GURGENIDZE;
pub mod antal_defense;
pub use antal_defense::ANTAL_DEFENSE;
pub mod balogh_defense;
pub use balogh_defense::BALOGH_DEFENSE;
pub mod english_rat;
pub use english_rat::ENGLISH_RAT;
pub mod fuller_gambit;
pub use fuller_gambit::FULLER_GAMBIT;
pub mod harmonist;
pub use harmonist::HARMONIST;
pub mod petruccioli_attack;
pub use petruccioli_attack::PETRUCCIOLI_ATTACK;
pub mod small_center_defense;
pub use small_center_defense::SMALL_CENTER_DEFENSE;
pub mod spike_attack;
pub use spike_attack::SPIKE_ATTACK;
