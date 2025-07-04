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
# use reco::book::pterodactyl_defense::SICILIAN;
assert_eq!(SICILIAN.original_name(), "Pterodactyl Defense: Sicilian");
```"#
)]
pub static SICILIAN: Variation = Variation {
    name: "Sicilian",
    parent: Some(&super::PTERODACTYL_DEFENSE),
    variations: &[
        &ANHANGUERA,
        &BENONI_GAMBIT,
        &PTERANODON,
        &QUETZALCOATLUS,
        &RHAMPHORHYNCHUS,
        &SIROCCOPTERYX,
        &UNPIN,
    ],
    lines: &[],
};
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod benoni_gambit;
pub use benoni_gambit::BENONI_GAMBIT;
pub mod pteranodon;
pub use pteranodon::PTERANODON;
pub mod quetzalcoatlus;
pub use quetzalcoatlus::QUETZALCOATLUS;
pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
pub mod siroccopteryx;
pub use siroccopteryx::SIROCCOPTERYX;
pub mod unpin;
pub use unpin::UNPIN;
