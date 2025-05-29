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
# use reco::book::kings_gambit_accepted::KIESERITZKY;
assert_eq!(KIESERITZKY.original_name(), "King's Gambit Accepted: Kieseritzky");
```"#)]
pub static KIESERITZKY: Variation = Variation {
    name: "Kieseritzky",
    variations: &[&POLERIO_DEFENSE,
&SALVIO_DEFENSE,
&LONG_WHIP_DEFENSE,
&RICE_GAMBIT],
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    lines: &[]
};pub mod polerio_defense;
pub use polerio_defense::POLERIO_DEFENSE;
pub mod salvio_defense;
pub use salvio_defense::SALVIO_DEFENSE;
pub mod long_whip_defense;
pub use long_whip_defense::LONG_WHIP_DEFENSE;
pub mod rice_gambit;
pub use rice_gambit::RICE_GAMBIT;
