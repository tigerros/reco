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
# use reco::book::kings_gambit_accepted::KIESERITZKY;
assert_eq!(KIESERITZKY.original_name(), "King's Gambit Accepted: Kieseritzky");
```"#
)]
pub static KIESERITZKY: Variation = Variation {
    name: "Kieseritzky",
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    variations: &[
        &LONG_WHIP_DEFENSE,
        &POLERIO_DEFENSE,
        &RICE_GAMBIT,
        &SALVIO_DEFENSE,
    ],
    lines: &[],
};
pub mod long_whip_defense;
pub use long_whip_defense::LONG_WHIP_DEFENSE;
pub mod polerio_defense;
pub use polerio_defense::POLERIO_DEFENSE;
pub mod rice_gambit;
pub use rice_gambit::RICE_GAMBIT;
pub mod salvio_defense;
pub use salvio_defense::SALVIO_DEFENSE;
