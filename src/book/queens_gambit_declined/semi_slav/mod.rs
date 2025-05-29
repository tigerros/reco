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
# use reco::book::queens_gambit_declined::SEMI_SLAV;
assert_eq!(SEMI_SLAV.original_name(), "Queen's Gambit Declined: Semi-Slav");
```"#)]
pub static SEMI_SLAV: Variation = Variation {
    name: "Semi-Slav",
    variations: &[&JUNGE_VARIATION,
&ABRAHAMS_VARIATION,
&KOOMEN_VARIATION],
    parent: Some(&super::QUEENS_GAMBIT_DECLINED),
    lines: &[]
};pub mod junge_variation;
pub use junge_variation::JUNGE_VARIATION;
pub mod abrahams_variation;
pub use abrahams_variation::ABRAHAMS_VARIATION;
pub mod koomen_variation;
pub use koomen_variation::KOOMEN_VARIATION;
