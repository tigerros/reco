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
# use reco::book::four_knights_game::spanish_variation::symmetrical_variation::TARRASCH_VARIATION;
assert_eq!(TARRASCH_VARIATION.original_name(), "Four Knights Game: Spanish Variation, Symmetrical Variation, Tarrasch Variation");
```"#
)]
pub static TARRASCH_VARIATION: Variation = Variation {
    name: "Tarrasch Variation",
    parent: Some(&super::SYMMETRICAL_VARIATION),
    variations: &[],
    lines: &[Line {
        parent: &TARRASCH_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<49>(),
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
                from: G8,
                capture: None,
                to: F6,
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
                from: B8,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F1,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: B4,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D3,
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
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C8,
                capture: None,
                to: E6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65029584701679360),
                    knight: Bitboard(39582420959232),
                    bishop: Bitboard(17875687440384),
                    rook: Bitboard(2377900603251621921),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7631134133053292544),
                    white: Bitboard(283739219817),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
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
