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
# use reco::book::modern_defense::SEMI_AVERBAKH_VARIATION;
assert_eq!(SEMI_AVERBAKH_VARIATION.original_name(), "Modern Defense: Semi-Averbakh Variation");
```"#
)]
pub static SEMI_AVERBAKH_VARIATION: Variation = Variation {
    name: "Semi-Averbakh Variation",
    parent: Some(&super::MODERN_DEFENSE),
    variations: &[
        &POLISH_VARIATION,
        &PTERODACTYL_VARIATION,
        &PTERODACTYL_VARIATION_ACCEPTED,
        &PTERODACTYL_VARIATION_DECLINED,
    ],
    lines: &[],
};
pub mod polish_variation;
pub use polish_variation::POLISH_VARIATION;
pub mod pterodactyl_variation;
pub use pterodactyl_variation::PTERODACTYL_VARIATION;
pub mod pterodactyl_variation_accepted;
pub use pterodactyl_variation_accepted::PTERODACTYL_VARIATION_ACCEPTED;
pub mod pterodactyl_variation_declined;
pub use pterodactyl_variation_declined::PTERODACTYL_VARIATION_DECLINED;
