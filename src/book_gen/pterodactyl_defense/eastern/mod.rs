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
# use reco::book::pterodactyl_defense::EASTERN;
assert_eq!(EASTERN.original_name(), "Pterodactyl Defense: Eastern");
```"#
)]
pub static EASTERN: Variation = Variation {
    name: "Eastern",
    parent: Some(&super::PTERODACTYL_DEFENSE),
    variations: &[
        &ANHANGUERA,
        &BENONI,
        &BENONI_PTERANODON,
        &BENONI_PTERODACTYL,
        &PTERANODON,
        &PTERODACTYL,
        &RHAMPHORHYNCHUS,
    ],
    lines: &[],
};
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod benoni;
pub use benoni::BENONI;
pub mod benoni_pteranodon;
pub use benoni_pteranodon::BENONI_PTERANODON;
pub mod benoni_pterodactyl;
pub use benoni_pterodactyl::BENONI_PTERODACTYL;
pub mod pteranodon;
pub use pteranodon::PTERANODON;
pub mod pterodactyl;
pub use pterodactyl::PTERODACTYL;
pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
