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
# use reco::book::dutch_defense::MANHATTAN_GAMBIT;
assert_eq!(MANHATTAN_GAMBIT.original_name(), "Dutch Defense: Manhattan Gambit");
```"#)]
pub static MANHATTAN_GAMBIT: Variation = Variation {
    name: "Manhattan Gambit",
    variations: &[&ANTI_LENINGRAD,
&ANTI_CLASSICAL_LINE,
&ANTI_MODERN,
&ANTI_STONEWALL],
    parent: Some(&super::DUTCH_DEFENSE),
    lines: &[]
};pub mod anti_leningrad;
pub use anti_leningrad::ANTI_LENINGRAD;
pub mod anti_classical_line;
pub use anti_classical_line::ANTI_CLASSICAL_LINE;
pub mod anti_modern;
pub use anti_modern::ANTI_MODERN;
pub mod anti_stonewall;
pub use anti_stonewall::ANTI_STONEWALL;
