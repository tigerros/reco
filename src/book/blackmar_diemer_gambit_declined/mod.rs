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
# use reco::book::BLACKMAR_DIEMER_GAMBIT_DECLINED;
assert_eq!(BLACKMAR_DIEMER_GAMBIT_DECLINED.original_name(), "Blackmar-Diemer Gambit Declined");
```"#)]
pub static BLACKMAR_DIEMER_GAMBIT_DECLINED: Variation = Variation {
    name: "Blackmar-Diemer Gambit Declined",
    variations: &[&GEDULT_DEFENSE,
&BROMBACHER_COUNTERGAMBIT,
&LANGEHEINICKE_DEFENSE,
&O_KELLY_DEFENSE,
&LAMB_DEFENSE,
&ELBERT_COUNTERGAMBIT,
&VIENNA_DEFENSE,
&WEINSBACH_DEFENSE],
    parent: None,
    lines: &[]
};pub mod gedult_defense;
pub use gedult_defense::GEDULT_DEFENSE;
pub mod brombacher_countergambit;
pub use brombacher_countergambit::BROMBACHER_COUNTERGAMBIT;
pub mod langeheinicke_defense;
pub use langeheinicke_defense::LANGEHEINICKE_DEFENSE;
pub mod o_kelly_defense;
pub use o_kelly_defense::O_KELLY_DEFENSE;
pub mod lamb_defense;
pub use lamb_defense::LAMB_DEFENSE;
pub mod elbert_countergambit;
pub use elbert_countergambit::ELBERT_COUNTERGAMBIT;
pub mod vienna_defense;
pub use vienna_defense::VIENNA_DEFENSE;
pub mod weinsbach_defense;
pub use weinsbach_defense::WEINSBACH_DEFENSE;
