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
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
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
# use reco::book::kings_indian_defense::FIANCHETTO_VARIATION;
assert_eq!(FIANCHETTO_VARIATION.original_name(), "King's Indian Defense: Fianchetto Variation");
```"#
)]
pub static FIANCHETTO_VARIATION: Variation = Variation {
    name: "Fianchetto Variation",
    parent: Some(&super::KINGS_INDIAN_DEFENSE),
    variations: &[
        &IMMEDIATE_FIANCHETTO,
        &KAVALEK_DEFENSE,
        &LONG_VARIATION,
        &UHLMANN_SZABO_SYSTEM,
        &DOUBLE_FIANCHETTO_ATTACK,
        &CLASSICAL_FIANCHETTO,
        &YUGOSLAV_SYSTEM,
        &DEBRECEN_DEFENSE,
        &DELAYED_FIANCHETTO,
        &YUGOSLAV_VARIATION,
        &LESSER_SIMAGIN_SPASSKY,
        &BENJAMIN_DEFENSE,
        &LARSEN_DEFENSE,
        &SIMAGIN_VARIATION,
        &PANNO_VARIATION,
        &CARLSBAD_VARIATION,
        &CLASSICAL_VARIATION,
        &PTERODACTYL_VARIATION,
        &HUNGARIAN_VARIATION,
        &CLASSICAL_MAIN_LINE,
    ],
    lines: &[],
};
pub mod immediate_fianchetto;
pub use immediate_fianchetto::IMMEDIATE_FIANCHETTO;
pub mod kavalek_defense;
pub use kavalek_defense::KAVALEK_DEFENSE;
pub mod long_variation;
pub use long_variation::LONG_VARIATION;
pub mod uhlmann_szabo_system;
pub use uhlmann_szabo_system::UHLMANN_SZABO_SYSTEM;
pub mod double_fianchetto_attack;
pub use double_fianchetto_attack::DOUBLE_FIANCHETTO_ATTACK;
pub mod classical_fianchetto;
pub use classical_fianchetto::CLASSICAL_FIANCHETTO;
pub mod yugoslav_system;
pub use yugoslav_system::YUGOSLAV_SYSTEM;
pub mod debrecen_defense;
pub use debrecen_defense::DEBRECEN_DEFENSE;
pub mod delayed_fianchetto;
pub use delayed_fianchetto::DELAYED_FIANCHETTO;
pub mod yugoslav_variation;
pub use yugoslav_variation::YUGOSLAV_VARIATION;
pub mod lesser_simagin_spassky;
pub use lesser_simagin_spassky::LESSER_SIMAGIN_SPASSKY;
pub mod benjamin_defense;
pub use benjamin_defense::BENJAMIN_DEFENSE;
pub mod larsen_defense;
pub use larsen_defense::LARSEN_DEFENSE;
pub mod simagin_variation;
pub use simagin_variation::SIMAGIN_VARIATION;
pub mod panno_variation;
pub use panno_variation::PANNO_VARIATION;
pub mod carlsbad_variation;
pub use carlsbad_variation::CARLSBAD_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod pterodactyl_variation;
pub use pterodactyl_variation::PTERODACTYL_VARIATION;
pub mod hungarian_variation;
pub use hungarian_variation::HUNGARIAN_VARIATION;
pub mod classical_main_line;
pub use classical_main_line::CLASSICAL_MAIN_LINE;
