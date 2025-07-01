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
# use reco::book::queens_gambit_accepted::CENTRAL_VARIATION;
assert_eq!(CENTRAL_VARIATION.original_name(), "Queen's Gambit Accepted: Central Variation");
```"#
)]
pub static CENTRAL_VARIATION: Variation = Variation {
    name: "Central Variation",
    parent: Some(&super::QUEENS_GAMBIT_ACCEPTED),
    variations: &[
        &ALEKHINE_SYSTEM,
        &GRECO_VARIATION,
        &MC_DONNELL_DEFENSE,
        &MODERN_DEFENSE,
        &RUBINSTEIN_DEFENSE,
    ],
    lines: &[],
};
pub mod alekhine_system;
pub use alekhine_system::ALEKHINE_SYSTEM;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
pub mod mc_donnell_defense;
pub use mc_donnell_defense::MC_DONNELL_DEFENSE;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
pub mod rubinstein_defense;
pub use rubinstein_defense::RUBINSTEIN_DEFENSE;
