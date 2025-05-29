#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::pterodactyl_defense::WESTERN;
assert_eq!(WESTERN.original_name(), "Pterodactyl Defense: Western");
```"#)]
pub static WESTERN: Variation = Variation {
    name: "Western",
    variations: &[&RHAMPHORHYNCHUS,
&PTERODACTYL,
&ANHANGUERA,
&SIROCCOPTERYX],
    parent: Some(&super::PTERODACTYL_DEFENSE),
    lines: &[]
};pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
pub mod pterodactyl;
pub use pterodactyl::PTERODACTYL;
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod siroccopteryx;
pub use siroccopteryx::SIROCCOPTERYX;
