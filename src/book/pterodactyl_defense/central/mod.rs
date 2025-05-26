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
/// Pterodactyl Defense: Central
pub static CENTRAL: Variation = Variation {
    name: "Central",
    parent: Some(&super::PTERODACTYL_DEFENSE),
    variations: &[
        &ANHANGUERA,
        &BENONI_BEEFEATER_PTERODACTYL,
        &BENONI_PTERODACTYL,
        &BENONI_QUETZALCOATLUS,
        &BOGOLUBOVIA,
        &QUETZALCOATLUS_GAMBIT,
    ],
    lines: &[],
};
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod benoni_beefeater_pterodactyl;
pub use benoni_beefeater_pterodactyl::BENONI_BEEFEATER_PTERODACTYL;
pub mod benoni_pterodactyl;
pub use benoni_pterodactyl::BENONI_PTERODACTYL;
pub mod benoni_quetzalcoatlus;
pub use benoni_quetzalcoatlus::BENONI_QUETZALCOATLUS;
pub mod bogolubovia;
pub use bogolubovia::BOGOLUBOVIA;
pub mod quetzalcoatlus_gambit;
pub use quetzalcoatlus_gambit::QUETZALCOATLUS_GAMBIT;
