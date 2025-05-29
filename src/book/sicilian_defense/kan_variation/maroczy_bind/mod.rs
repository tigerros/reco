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
# use reco::book::sicilian_defense::kan_variation::MAROCZY_BIND;
assert_eq!(MAROCZY_BIND.original_name(), "Sicilian Defense: Kan Variation, Maróczy Bind");
```"#)]
pub static MAROCZY_BIND: Variation = Variation {
    name: "Maróczy Bind",
    variations: &[&BRONSTEIN_VARIATION,
&HEDGEHOG_VARIATION,
&RETI_VARIATION],
    parent: Some(&super::KAN_VARIATION),
    lines: &[]
};pub mod bronstein_variation;
pub use bronstein_variation::BRONSTEIN_VARIATION;
pub mod hedgehog_variation;
pub use hedgehog_variation::HEDGEHOG_VARIATION;
pub mod reti_variation;
pub use reti_variation::RETI_VARIATION;
