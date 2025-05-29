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
# use reco::book::reti_opening::ANGLO_SLAV_VARIATION;
assert_eq!(ANGLO_SLAV_VARIATION.original_name(), "Réti Opening: Anglo-Slav Variation");
```"#)]
pub static ANGLO_SLAV_VARIATION: Variation = Variation {
    name: "Anglo-Slav Variation",
    variations: &[&NEW_YORK_SYSTEM,
&TORRE_SYSTEM,
&CAPABLANCA_VARIATION,
&BOGOLJUBOW_VARIATION,
&LONDON_DEFENSIVE_SYSTEM,
&BLED_VARIATION],
    parent: Some(&super::RETI_OPENING),
    lines: &[]
};pub mod new_york_system;
pub use new_york_system::NEW_YORK_SYSTEM;
pub mod torre_system;
pub use torre_system::TORRE_SYSTEM;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod london_defensive_system;
pub use london_defensive_system::LONDON_DEFENSIVE_SYSTEM;
pub mod bled_variation;
pub use bled_variation::BLED_VARIATION;
