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
# use reco::book::sicilian_defense::closed::ANTI_SVESHNIKOV_VARIATION;
assert_eq!(ANTI_SVESHNIKOV_VARIATION.original_name(), "Sicilian Defense: Closed, Anti-Sveshnikov Variation");
```"#)]
pub static ANTI_SVESHNIKOV_VARIATION: Variation = Variation {
    name: "Anti-Sveshnikov Variation",
    variations: &[&WITH_D6,
&KHARLOV_KRAMNIK_LINE,
&WITH_NF6],
    parent: Some(&super::CLOSED),
    lines: &[]
};pub mod with_d6;
pub use with_d6::WITH_D6;
pub mod kharlov_kramnik_line;
pub use kharlov_kramnik_line::KHARLOV_KRAMNIK_LINE;
pub mod with_nf6;
pub use with_nf6::WITH_NF6;
