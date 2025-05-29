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
# use reco::book::pterodactyl_defense::CENTRAL;
assert_eq!(CENTRAL.original_name(), "Pterodactyl Defense: Central");
```"#)]
pub static CENTRAL: Variation = Variation {
    name: "Central",
    variations: &[&BENONI_BEEFEATER_PTERODACTYL,
&QUETZALCOATLUS_GAMBIT,
&ANHANGUERA,
&BENONI_QUETZALCOATLUS,
&BOGOLUBOVIA,
&BENONI_PTERODACTYL],
    parent: Some(&super::PTERODACTYL_DEFENSE),
    lines: &[]
};pub mod benoni_beefeater_pterodactyl;
pub use benoni_beefeater_pterodactyl::BENONI_BEEFEATER_PTERODACTYL;
pub mod quetzalcoatlus_gambit;
pub use quetzalcoatlus_gambit::QUETZALCOATLUS_GAMBIT;
pub mod anhanguera;
pub use anhanguera::ANHANGUERA;
pub mod benoni_quetzalcoatlus;
pub use benoni_quetzalcoatlus::BENONI_QUETZALCOATLUS;
pub mod bogolubovia;
pub use bogolubovia::BOGOLUBOVIA;
pub mod benoni_pterodactyl;
pub use benoni_pterodactyl::BENONI_PTERODACTYL;
