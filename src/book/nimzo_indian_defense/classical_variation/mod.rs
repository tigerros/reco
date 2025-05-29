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
# use reco::book::nimzo_indian_defense::CLASSICAL_VARIATION;
assert_eq!(CLASSICAL_VARIATION.original_name(), "Nimzo-Indian Defense: Classical Variation");
```"#)]
pub static CLASSICAL_VARIATION: Variation = Variation {
    name: "Classical Variation",
    variations: &[&KERES_DEFENSE,
&NOA_VARIATION,
&MILNER_BARRY_VARIATION,
&ZURICH_VARIATION,
&BELYAVSKY_GAMBIT,
&BERLIN_VARIATION,
&ROMANISHIN_GAMBIT,
&MODERN_VARIATION,
&VITOLINS_ADORJAN_GAMBIT],
    parent: Some(&super::NIMZO_INDIAN_DEFENSE),
    lines: &[Line {
    code: Code {
        volume: Volume::E,
        category: Category::new_static::<32>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E7,
        capture: None,
        to: E6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: C2,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111821280000),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303424512),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(11524482748056076288),
                white: Bitboard(201652213)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 3,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod keres_defense;
pub use keres_defense::KERES_DEFENSE;
pub mod noa_variation;
pub use noa_variation::NOA_VARIATION;
pub mod milner_barry_variation;
pub use milner_barry_variation::MILNER_BARRY_VARIATION;
pub mod zurich_variation;
pub use zurich_variation::ZURICH_VARIATION;
pub mod belyavsky_gambit;
pub use belyavsky_gambit::BELYAVSKY_GAMBIT;
pub mod berlin_variation;
pub use berlin_variation::BERLIN_VARIATION;
pub mod romanishin_gambit;
pub use romanishin_gambit::ROMANISHIN_GAMBIT;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod vitolins_adorjan_gambit;
pub use vitolins_adorjan_gambit::VITOLINS_ADORJAN_GAMBIT;
