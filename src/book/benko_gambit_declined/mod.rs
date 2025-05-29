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
# use reco::book::BENKO_GAMBIT_DECLINED;
assert_eq!(BENKO_GAMBIT_DECLINED.original_name(), "Benko Gambit Declined");
```"#)]
pub static BENKO_GAMBIT_DECLINED: Variation = Variation {
    name: "Benko Gambit Declined",
    variations: &[&QUIET_LINE,
&SOSONKO_VARIATION,
&HJORRING_COUNTERGAMBIT,
&BISHOP_ATTACK,
&MAIN_LINE,
&PSEUDO_SAMISCH],
    parent: None,
    lines: &[]
};pub mod quiet_line;
pub use quiet_line::QUIET_LINE;
pub mod sosonko_variation;
pub use sosonko_variation::SOSONKO_VARIATION;
pub mod hjorring_countergambit;
pub use hjorring_countergambit::HJORRING_COUNTERGAMBIT;
pub mod bishop_attack;
pub use bishop_attack::BISHOP_ATTACK;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod pseudo_samisch;
pub use pseudo_samisch::PSEUDO_SAMISCH;
