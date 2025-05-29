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
# use reco::book::pterodactyl_defense::SICILIAN;
assert_eq!(SICILIAN.original_name(), "Pterodactyl Defense: Sicilian");
```"#)]
pub static SICILIAN: Variation = Variation {
    name: "Sicilian",
    variations: &[&PTERANODON,
&BENONI_GAMBIT,
&RHAMPHORHYNCHUS,
&SIROCCOPTERYX,
&ANHANGUERA,
&UNPIN,
&QUETZALCOATLUS],
    parent: Some(&super::PTERODACTYL_DEFENSE),
    lines: &[]
};pub mod pteranodon;
pub use pteranodon::PTERANODON;
pub mod benoni_gambit;
pub use benoni_gambit::BENONI_GAMBIT;
pub mod rhamphorhynchus;
pub use rhamphorhynchus::RHAMPHORHYNCHUS;
pub mod siroccopteryx;
pub use siroccopteryx::SIROCCOPTERYX;
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod unpin;
pub use unpin::UNPIN;
pub mod quetzalcoatlus;
pub use quetzalcoatlus::QUETZALCOATLUS;
