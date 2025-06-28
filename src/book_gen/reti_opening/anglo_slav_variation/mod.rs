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
# use reco::book::reti_opening::ANGLO_SLAV_VARIATION;
assert_eq!(ANGLO_SLAV_VARIATION.original_name(), "Réti Opening: Anglo-Slav Variation");
```"#
)]
pub static ANGLO_SLAV_VARIATION: Variation = Variation {
    name: "Anglo-Slav Variation",
    parent: Some(&super::RETI_OPENING),
    variations: &[
        &BLED_VARIATION,
        &BOGOLJUBOW_VARIATION,
        &CAPABLANCA_VARIATION,
        &LONDON_DEFENSIVE_SYSTEM,
        &NEW_YORK_SYSTEM,
        &TORRE_SYSTEM,
    ],
    lines: &[],
};
pub mod bled_variation;
pub use bled_variation::BLED_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod london_defensive_system;
pub use london_defensive_system::LONDON_DEFENSIVE_SYSTEM;
pub mod new_york_system;
pub use new_york_system::NEW_YORK_SYSTEM;
pub mod torre_system;
pub use torre_system::TORRE_SYSTEM;
