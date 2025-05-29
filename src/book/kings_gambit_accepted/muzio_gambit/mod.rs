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
# use reco::book::kings_gambit_accepted::MUZIO_GAMBIT;
assert_eq!(MUZIO_GAMBIT.original_name(), "King's Gambit Accepted: Muzio Gambit");
```"#)]
pub static MUZIO_GAMBIT: Variation = Variation {
    name: "Muzio Gambit",
    variations: &[&HOLLOWAY_DEFENSE,
&KLING_AND_HORWITZ_COUNTERATTACK,
&BRENTANO_DEFENSE,
&SARRATT_DEFENSE,
&WILD_MUZIO_GAMBIT],
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    lines: &[]
};pub mod holloway_defense;
pub use holloway_defense::HOLLOWAY_DEFENSE;
pub mod kling_and_horwitz_counterattack;
pub use kling_and_horwitz_counterattack::KLING_AND_HORWITZ_COUNTERATTACK;
pub mod brentano_defense;
pub use brentano_defense::BRENTANO_DEFENSE;
pub mod sarratt_defense;
pub use sarratt_defense::SARRATT_DEFENSE;
pub mod wild_muzio_gambit;
pub use wild_muzio_gambit::WILD_MUZIO_GAMBIT;
