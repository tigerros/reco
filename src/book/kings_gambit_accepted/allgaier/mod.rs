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
# use reco::book::kings_gambit_accepted::ALLGAIER;
assert_eq!(ALLGAIER.original_name(), "King's Gambit Accepted: Allgaier");
```"#)]
pub static ALLGAIER: Variation = Variation {
    name: "Allgaier",
    variations: &[&BLACKBURNE_GAMBIT,
&URUSOV_ATTACK,
&COOK_VARIATION,
&HORNY_DEFENSE,
&SCHLECHTER_DEFENSE],
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    lines: &[]
};pub mod blackburne_gambit;
pub use blackburne_gambit::BLACKBURNE_GAMBIT;
pub mod urusov_attack;
pub use urusov_attack::URUSOV_ATTACK;
pub mod cook_variation;
pub use cook_variation::COOK_VARIATION;
pub mod horny_defense;
pub use horny_defense::HORNY_DEFENSE;
pub mod schlechter_defense;
pub use schlechter_defense::SCHLECHTER_DEFENSE;
