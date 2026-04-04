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
# use reco::book::italian_game::classical_variation::GRECO_GAMBIT;
assert_eq!(GRECO_GAMBIT.original_name(), "Italian Game: Classical Variation, Greco Gambit");
```"#
)]
pub static GRECO_GAMBIT: Variation = Variation {
    name: "Greco Gambit",
    parent: Some(&super::CLASSICAL_VARIATION),
    variations: &[
        &ANDERSSEN_VARIATION,
        &DUBOV_ITALIAN,
        &GRECO_VARIATION,
        &MAIN_LINE,
        &MODERN_LINE,
        &MOELLER_BAYONET_ATTACK,
        &MOELLER_THERKATZ_ATTACK,
        &TRADITIONAL_LINE,
    ],
    lines: &[],
};
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod dubov_italian;
pub use dubov_italian::DUBOV_ITALIAN;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod modern_line;
pub use modern_line::MODERN_LINE;
pub mod moeller_bayonet_attack;
pub use moeller_bayonet_attack::MOELLER_BAYONET_ATTACK;
pub mod moeller_therkatz_attack;
pub use moeller_therkatz_attack::MOELLER_THERKATZ_ATTACK;
pub mod traditional_line;
pub use traditional_line::TRADITIONAL_LINE;
