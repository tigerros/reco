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
# use reco::book::pterodactyl_defense::FIANCHETTO;
assert_eq!(FIANCHETTO.original_name(), "Pterodactyl Defense: Fianchetto");
```"#
)]
pub static FIANCHETTO: Variation = Variation {
    name: "Fianchetto",
    parent: Some(&super::PTERODACTYL_DEFENSE),
    variations: &[
        &KING_PTERODACTYL,
        &QUEEN_BENONI_PTERODACTYL,
        &QUEEN_PTERANODON,
        &QUEEN_PTERODACTYL,
        &RHAMPHORHYNCHUS,
    ],
    lines: &[],
};
pub mod king_pterodactyl;
pub use king_pterodactyl::KING_PTERODACTYL;
pub mod queen_benoni_pterodactyl;
pub use queen_benoni_pterodactyl::QUEEN_BENONI_PTERODACTYL;
pub mod queen_pteranodon;
pub use queen_pteranodon::QUEEN_PTERANODON;
pub mod queen_pterodactyl;
pub use queen_pterodactyl::QUEEN_PTERODACTYL;
pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
