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
# use reco::book::alekhine_defense::MODERN_VARIATION;
assert_eq!(MODERN_VARIATION.original_name(), "Alekhine Defense: Modern Variation");
```"#)]
pub static MODERN_VARIATION: Variation = Variation {
    name: "Modern Variation",
    variations: &[&LARSEN_HAAKERT_VARIATION,
&VITOLINS_ATTACK,
&LARSEN_VARIATION,
&MAIN_LINE,
&ALBURT_VARIATION,
&PANOV_VARIATION,
&ALEKHINE_VARIATION,
&SCHMID_VARIATION,
&ALEKHINE_GAMBIT,
&KERES_VARIATION,
&FLOHR_VARIATION],
    parent: Some(&super::ALEKHINE_DEFENSE),
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<4>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E4,
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
        from: E4,
        capture: None,
        to: E5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D7,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(69533184194307840),
                knight: Bitboard(144115222437691394),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13832533610944528384),
                white: Bitboard(68855850943)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod larsen_haakert_variation;
pub use larsen_haakert_variation::LARSEN_HAAKERT_VARIATION;
pub mod vitolins_attack;
pub use vitolins_attack::VITOLINS_ATTACK;
pub mod larsen_variation;
pub use larsen_variation::LARSEN_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod alburt_variation;
pub use alburt_variation::ALBURT_VARIATION;
pub mod panov_variation;
pub use panov_variation::PANOV_VARIATION;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod schmid_variation;
pub use schmid_variation::SCHMID_VARIATION;
pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;
pub mod keres_variation;
pub use keres_variation::KERES_VARIATION;
pub mod flohr_variation;
pub use flohr_variation::FLOHR_VARIATION;
