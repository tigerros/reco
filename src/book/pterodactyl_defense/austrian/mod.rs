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
# use reco::book::pterodactyl_defense::AUSTRIAN;
assert_eq!(AUSTRIAN.original_name(), "Pterodactyl Defense: Austrian");
```"#)]
pub static AUSTRIAN: Variation = Variation {
    name: "Austrian",
    variations: &[&AUSTRIADACTYLUS_WESTERN,
&GRAND_PRIX_PTERODACTYL,
&PTERANODON],
    parent: Some(&super::PTERODACTYL_DEFENSE),
    lines: &[]
};pub mod austriadactylus_western;
pub use austriadactylus_western::AUSTRIADACTYLUS_WESTERN;
pub mod grand_prix_pterodactyl;
pub use grand_prix_pterodactyl::GRAND_PRIX_PTERODACTYL;
pub mod pteranodon;
pub use pteranodon::PTERANODON;
