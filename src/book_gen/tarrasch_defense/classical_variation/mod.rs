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
# use reco::book::tarrasch_defense::CLASSICAL_VARIATION;
assert_eq!(CLASSICAL_VARIATION.original_name(), "Tarrasch Defense: Classical Variation");
```"#
)]
pub static CLASSICAL_VARIATION: Variation = Variation {
    name: "Classical Variation",
    parent: Some(&super::TARRASCH_DEFENSE),
    variations: &[
        &ADVANCE_VARIATION,
        &BOGOLJUBOW_VARIATION,
        &CARLSBAD_VARIATION,
        &CHANDLER_VARIATION,
        &CLASSICAL_TARRASCH_GAMBIT,
        &ENDGAME_VARIATION,
        &MAIN_LINE,
        &PETURSSON_VARIATION,
        &RETI_VARIATION,
        &SPASSKY_VARIATION,
    ],
    lines: &[Line {
        parent: &CLASSICAL_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<34>(),
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
                role: Pawn,
                from: D7,
                capture: None,
                to: D5,
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
                role: Pawn,
                from: C7,
                capture: None,
                to: C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C4,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E6,
                capture: Some(Pawn),
                to: D5,
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
                from: G2,
                capture: None,
                to: G3,
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
                role: Bishop,
                from: F1,
                capture: None,
                to: G2,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Castle { king: E8, rook: H8 },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63894871391384320),
                    knight: Bitboard(39582420959232),
                    bishop: Bitboard(292733975779098628),
                    rook: Bitboard(2377900603251621921),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7922715803433041920),
                    white: Bitboard(140833645),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 5,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                fullmoves
            } else {
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
        },
    }],
};
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod carlsbad_variation;
pub use carlsbad_variation::CARLSBAD_VARIATION;
pub mod chandler_variation;
pub use chandler_variation::CHANDLER_VARIATION;
pub mod classical_tarrasch_gambit;
pub use classical_tarrasch_gambit::CLASSICAL_TARRASCH_GAMBIT;
pub mod endgame_variation;
pub use endgame_variation::ENDGAME_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod petursson_variation;
pub use petursson_variation::PETURSSON_VARIATION;
pub mod reti_variation;
pub use reti_variation::RETI_VARIATION;
pub mod spassky_variation;
pub use spassky_variation::SPASSKY_VARIATION;
