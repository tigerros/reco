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
# use reco::book::KINGS_GAMBIT_DECLINED;
assert_eq!(KINGS_GAMBIT_DECLINED.original_name(), "King's Gambit Declined");
```"#)]
pub static KINGS_GAMBIT_DECLINED: Variation = Variation {
    name: "King's Gambit Declined",
    variations: &[&PETROVS_DEFENSE,
&CLASSICAL,
&KEENE_DEFENSE,
&SENECHAUD_COUNTERGAMBIT,
&ZILBERMINTS_DOUBLE_GAMBIT,
&KEENES_DEFENSE,
&ZILBERMINTS_DOUBLE_COUNTERGAMBIT,
&FALKBEER_COUNTERGAMBIT_ACCEPTED,
&CLASSICAL_VARIATION,
&MILES_DEFENSE,
&NORWALDE_VARIATION,
&QUEENS_KNIGHT_DEFENSE,
&PANTELDAKIS_COUNTERGAMBIT,
&SOLLER_ZILBERMINTS_GAMBIT,
&FALKBEER_COUNTERGAMBIT,
&MAFIA_DEFENSE,
&HOBBS_ZILBERMINTS_GAMBIT],
    parent: None,
    lines: &[]
};pub mod petrovs_defense;
pub use petrovs_defense::PETROVS_DEFENSE;
pub mod classical;
pub use classical::CLASSICAL;
pub mod keene_defense;
pub use keene_defense::KEENE_DEFENSE;
pub mod senechaud_countergambit;
pub use senechaud_countergambit::SENECHAUD_COUNTERGAMBIT;
pub mod zilbermints_double_gambit;
pub use zilbermints_double_gambit::ZILBERMINTS_DOUBLE_GAMBIT;
pub mod keenes_defense;
pub use keenes_defense::KEENES_DEFENSE;
pub mod zilbermints_double_countergambit;
pub use zilbermints_double_countergambit::ZILBERMINTS_DOUBLE_COUNTERGAMBIT;
pub mod falkbeer_countergambit_accepted;
pub use falkbeer_countergambit_accepted::FALKBEER_COUNTERGAMBIT_ACCEPTED;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod miles_defense;
pub use miles_defense::MILES_DEFENSE;
pub mod norwalde_variation;
pub use norwalde_variation::NORWALDE_VARIATION;
pub mod queens_knight_defense;
pub use queens_knight_defense::QUEENS_KNIGHT_DEFENSE;
pub mod panteldakis_countergambit;
pub use panteldakis_countergambit::PANTELDAKIS_COUNTERGAMBIT;
pub mod soller_zilbermints_gambit;
pub use soller_zilbermints_gambit::SOLLER_ZILBERMINTS_GAMBIT;
pub mod falkbeer_countergambit;
pub use falkbeer_countergambit::FALKBEER_COUNTERGAMBIT;
pub mod mafia_defense;
pub use mafia_defense::MAFIA_DEFENSE;
pub mod hobbs_zilbermints_gambit;
pub use hobbs_zilbermints_gambit::HOBBS_ZILBERMINTS_GAMBIT;
