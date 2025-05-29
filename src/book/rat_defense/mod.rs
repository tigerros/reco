#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::RAT_DEFENSE;
assert_eq!(RAT_DEFENSE.original_name(), "Rat Defense");
```"#)]
pub static RAT_DEFENSE: Variation = Variation {
    name: "Rat Defense",
    variations: &[&ENGLISH_RAT,
&HARMONIST,
&FULLER_GAMBIT,
&SMALL_CENTER_DEFENSE,
&BALOGH_DEFENSE,
&ACCELERATED_GURGENIDZE,
&SPIKE_ATTACK,
&PETRUCCIOLI_ATTACK,
&ANTAL_DEFENSE],
    parent: None,
    lines: &[]
};pub mod english_rat;
pub use english_rat::ENGLISH_RAT;
pub mod harmonist;
pub use harmonist::HARMONIST;
pub mod fuller_gambit;
pub use fuller_gambit::FULLER_GAMBIT;
pub mod small_center_defense;
pub use small_center_defense::SMALL_CENTER_DEFENSE;
pub mod balogh_defense;
pub use balogh_defense::BALOGH_DEFENSE;
pub mod accelerated_gurgenidze;
pub use accelerated_gurgenidze::ACCELERATED_GURGENIDZE;
pub mod spike_attack;
pub use spike_attack::SPIKE_ATTACK;
pub mod petruccioli_attack;
pub use petruccioli_attack::PETRUCCIOLI_ATTACK;
pub mod antal_defense;
pub use antal_defense::ANTAL_DEFENSE;
