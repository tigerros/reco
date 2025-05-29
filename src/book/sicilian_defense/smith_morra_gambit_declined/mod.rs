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
# use reco::book::sicilian_defense::SMITH_MORRA_GAMBIT_DECLINED;
assert_eq!(SMITH_MORRA_GAMBIT_DECLINED.original_name(), "Sicilian Defense: Smith-Morra Gambit Declined");
```"#)]
pub static SMITH_MORRA_GAMBIT_DECLINED: Variation = Variation {
    name: "Smith-Morra Gambit Declined",
    variations: &[&WING_FORMATION,
&CENTER_FORMATION,
&SCANDINAVIAN_FORMATION,
&DUBOIS_VARIATION,
&PUSH_VARIATION,
&ALAPIN_FORMATION],
    parent: Some(&super::SICILIAN_DEFENSE),
    lines: &[]
};pub mod wing_formation;
pub use wing_formation::WING_FORMATION;
pub mod center_formation;
pub use center_formation::CENTER_FORMATION;
pub mod scandinavian_formation;
pub use scandinavian_formation::SCANDINAVIAN_FORMATION;
pub mod dubois_variation;
pub use dubois_variation::DUBOIS_VARIATION;
pub mod push_variation;
pub use push_variation::PUSH_VARIATION;
pub mod alapin_formation;
pub use alapin_formation::ALAPIN_FORMATION;
