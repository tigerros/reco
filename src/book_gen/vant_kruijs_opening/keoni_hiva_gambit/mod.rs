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
# use reco::book::vant_kruijs_opening::KEONI_HIVA_GAMBIT;
assert_eq!(KEONI_HIVA_GAMBIT.original_name(), "Van't Kruijs Opening: Keoni-Hiva Gambit");
```"#
)]
pub static KEONI_HIVA_GAMBIT: Variation = Variation {
    name: "Keoni-Hiva Gambit",
    parent: Some(&super::VANT_KRUIJS_OPENING),
    variations: &[&EKOLU_VARIATION, &AKAHI_VARIATION, &ALUA_VARIATION],
    lines: &[],
};
pub mod ekolu_variation;
pub use ekolu_variation::EKOLU_VARIATION;
pub mod akahi_variation;
pub use akahi_variation::AKAHI_VARIATION;
pub mod alua_variation;
pub use alua_variation::ALUA_VARIATION;
