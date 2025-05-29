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
# use reco::book::COLLE_SYSTEM;
assert_eq!(COLLE_SYSTEM.original_name(), "Colle System");
```"#)]
pub static COLLE_SYSTEM: Variation = Variation {
    name: "Colle System",
    variations: &[&SIROCCOPTERYX_VARIATION,
&RHAMPHORHYNCHUS_VARIATION,
&PTERODACTYL_VARIATION],
    parent: None,
    lines: &[]
};pub mod siroccopteryx_variation;
pub use siroccopteryx_variation::SIROCCOPTERYX_VARIATION;
pub mod rhamphorhynchus_variation;
pub use rhamphorhynchus_variation::RHAMPHORHYNCHUS_VARIATION;
pub mod pterodactyl_variation;
pub use pterodactyl_variation::PTERODACTYL_VARIATION;
