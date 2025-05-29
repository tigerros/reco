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
# use reco::book::kings_gambit_declined::CLASSICAL;
assert_eq!(CLASSICAL.original_name(), "King's Gambit Declined: Classical");
```"#)]
pub static CLASSICAL: Variation = Variation {
    name: "Classical",
    variations: &[&HANHAM_VARIATION,
&RETI_VARIATION,
&SOLDATENKOV_VARIATION,
&SVENONIUS_VARIATION],
    parent: Some(&super::KINGS_GAMBIT_DECLINED),
    lines: &[]
};pub mod hanham_variation;
pub use hanham_variation::HANHAM_VARIATION;
pub mod reti_variation;
pub use reti_variation::RETI_VARIATION;
pub mod soldatenkov_variation;
pub use soldatenkov_variation::SOLDATENKOV_VARIATION;
pub mod svenonius_variation;
pub use svenonius_variation::SVENONIUS_VARIATION;
