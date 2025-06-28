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
# use reco::book::sicilian_defense::closed::ANTI_SVESHNIKOV_VARIATION;
assert_eq!(ANTI_SVESHNIKOV_VARIATION.original_name(), "Sicilian Defense: Closed, Anti-Sveshnikov Variation");
```"#
)]
pub static ANTI_SVESHNIKOV_VARIATION: Variation = Variation {
    name: "Anti-Sveshnikov Variation",
    parent: Some(&super::CLOSED),
    variations: &[&KHARLOV_KRAMNIK_LINE, &WITH_NF6, &WITH_D6],
    lines: &[],
};
pub mod kharlov_kramnik_line;
pub use kharlov_kramnik_line::KHARLOV_KRAMNIK_LINE;
pub mod with_nf6;
pub use with_nf6::WITH_NF6;
pub mod with_d6;
pub use with_d6::WITH_D6;
