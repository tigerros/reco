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
# use reco::book::vienna_game::VIENNA_GAMBIT;
assert_eq!(VIENNA_GAMBIT.original_name(), "Vienna Game: Vienna Gambit");
```"#)]
pub static VIENNA_GAMBIT: Variation = Variation {
    name: "Vienna Gambit",
    variations: &[&MAIN_LINE,
&BREYER_VARIATION,
&BARDELEBEN_VARIATION,
&KAUFMANN_VARIATION,
&MODERN_VARIATION,
&PAULSEN_ATTACK,
&WURZBURGER_TRAP,
&STEINITZ_VARIATION],
    parent: Some(&super::VIENNA_GAME),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<29>()
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
        role: Pawn,
        from: E7,
        capture: None,
        to: E5,
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
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588958682880),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13830308233769648128),
                white: Bitboard(805621757)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod bardeleben_variation;
pub use bardeleben_variation::BARDELEBEN_VARIATION;
pub mod kaufmann_variation;
pub use kaufmann_variation::KAUFMANN_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod paulsen_attack;
pub use paulsen_attack::PAULSEN_ATTACK;
pub mod wurzburger_trap;
pub use wurzburger_trap::WURZBURGER_TRAP;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
