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
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::indian_defense::ANTI_GRUNFELD;
assert_eq!(ANTI_GRUNFELD.original_name(), "Indian Defense: Anti-Grünfeld");
```"#
)]
pub static ANTI_GRUNFELD: Variation = Variation {
    name: "Anti-Grünfeld",
    parent: Some(&super::INDIAN_DEFENSE),
    variations: &[
        &ADORJAN_GAMBIT,
        &BASMAN_WILLIAMS_ATTACK,
        &ADVANCE_VARIATION,
        &ALEKHINE_VARIATION,
    ],
    lines: &[],
};
pub mod adorjan_gambit;
pub use adorjan_gambit::ADORJAN_GAMBIT;
pub mod basman_williams_attack;
pub use basman_williams_attack::BASMAN_WILLIAMS_ATTACK;
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
