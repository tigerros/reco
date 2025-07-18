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
use core::unreachable;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use deranged::RangedU8;
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
# use reco::book::NEO_GRUNFELD_DEFENSE;
assert_eq!(NEO_GRUNFELD_DEFENSE.original_name(), "Neo-Grünfeld Defense");
```"#
)]
pub static NEO_GRUNFELD_DEFENSE: Variation = Variation {
    name: "Neo-Grünfeld Defense",
    parent: None,
    variations: &[
        &CLASSICAL_VARIATION,
        &DELAYED_EXCHANGE_VARIATION,
        &EXCHANGE_VARIATION,
        &GOGLIDZE_ATTACK,
        &ULTRA_DELAYED_EXCHANGE_VARIATION,
        &WITH_NF3,
        &WITH_G3,
    ],
    lines: &[],
};
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod delayed_exchange_variation;
pub use delayed_exchange_variation::DELAYED_EXCHANGE_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod goglidze_attack;
pub use goglidze_attack::GOGLIDZE_ATTACK;
pub mod ultra_delayed_exchange_variation;
pub use ultra_delayed_exchange_variation::ULTRA_DELAYED_EXCHANGE_VARIATION;
pub mod with_nf3;
pub use with_nf3::WITH_NF3;
pub mod with_g3;
pub use with_g3::WITH_G3;
