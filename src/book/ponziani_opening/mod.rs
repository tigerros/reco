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
# use reco::book::PONZIANI_OPENING;
assert_eq!(PONZIANI_OPENING.original_name(), "Ponziani Opening");
```"#)]
pub static PONZIANI_OPENING: Variation = Variation {
    name: "Ponziani Opening",
    variations: &[&ROMANISHIN_VARIATION,
&SPANISH_VARIATION,
&NEUMANN_GAMBIT,
&VUKOVIC_GAMBIT,
&PONZIANI_COUNTERGAMBIT,
&RETI_VARIATION,
&JAENISCH_COUNTERATTACK,
&LEONHARDT_VARIATION,
&CARO_GAMBIT,
&STEINITZ_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<44>()
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
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588422081280),
                knight: Bitboard(4611690416475996162),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18297848277795602432),
                white: Bitboard(270855103)
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
};pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;
pub mod spanish_variation;
pub use spanish_variation::SPANISH_VARIATION;
pub mod neumann_gambit;
pub use neumann_gambit::NEUMANN_GAMBIT;
pub mod vukovic_gambit;
pub use vukovic_gambit::VUKOVIC_GAMBIT;
pub mod ponziani_countergambit;
pub use ponziani_countergambit::PONZIANI_COUNTERGAMBIT;
pub mod reti_variation;
pub use reti_variation::RETI_VARIATION;
pub mod jaenisch_counterattack;
pub use jaenisch_counterattack::JAENISCH_COUNTERATTACK;
pub mod leonhardt_variation;
pub use leonhardt_variation::LEONHARDT_VARIATION;
pub mod caro_gambit;
pub use caro_gambit::CARO_GAMBIT;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
