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
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// King's Gambit Declined
pub static KINGS_GAMBIT_DECLINED: Variation = Variation {
    name: "King's Gambit Declined",
    parent: None,
    variations: &[
        &FALKBEER_COUNTERGAMBIT_ACCEPTED,
        &MAFIA_DEFENSE,
        &HOBBS_ZILBERMINTS_GAMBIT,
        &MILES_DEFENSE,
        &PETROVS_DEFENSE,
        &NORWALDE_VARIATION,
        &KEENES_DEFENSE,
        &ZILBERMINTS_DOUBLE_COUNTERGAMBIT,
        &KEENE_DEFENSE,
        &SOLLER_ZILBERMINTS_GAMBIT,
        &ZILBERMINTS_DOUBLE_GAMBIT,
        &QUEENS_KNIGHT_DEFENSE,
        &FALKBEER_COUNTERGAMBIT,
        &CLASSICAL,
        &PANTELDAKIS_COUNTERGAMBIT,
        &CLASSICAL_VARIATION,
        &SENECHAUD_COUNTERGAMBIT,
    ],
    lines: &[],
};
pub mod falkbeer_countergambit_accepted;
pub use falkbeer_countergambit_accepted::FALKBEER_COUNTERGAMBIT_ACCEPTED;
pub mod mafia_defense;
pub use mafia_defense::MAFIA_DEFENSE;
pub mod hobbs_zilbermints_gambit;
pub use hobbs_zilbermints_gambit::HOBBS_ZILBERMINTS_GAMBIT;
pub mod miles_defense;
pub use miles_defense::MILES_DEFENSE;
pub mod petrovs_defense;
pub use petrovs_defense::PETROVS_DEFENSE;
pub mod norwalde_variation;
pub use norwalde_variation::NORWALDE_VARIATION;
pub mod keenes_defense;
pub use keenes_defense::KEENES_DEFENSE;
pub mod zilbermints_double_countergambit;
pub use zilbermints_double_countergambit::ZILBERMINTS_DOUBLE_COUNTERGAMBIT;
pub mod keene_defense;
pub use keene_defense::KEENE_DEFENSE;
pub mod soller_zilbermints_gambit;
pub use soller_zilbermints_gambit::SOLLER_ZILBERMINTS_GAMBIT;
pub mod zilbermints_double_gambit;
pub use zilbermints_double_gambit::ZILBERMINTS_DOUBLE_GAMBIT;
pub mod queens_knight_defense;
pub use queens_knight_defense::QUEENS_KNIGHT_DEFENSE;
pub mod falkbeer_countergambit;
pub use falkbeer_countergambit::FALKBEER_COUNTERGAMBIT;
pub mod classical;
pub use classical::CLASSICAL;
pub mod panteldakis_countergambit;
pub use panteldakis_countergambit::PANTELDAKIS_COUNTERGAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod senechaud_countergambit;
pub use senechaud_countergambit::SENECHAUD_COUNTERGAMBIT;
