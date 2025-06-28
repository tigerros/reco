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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
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
# use reco::book::ruy_lopez::marshall_attack::RE3_VARIATION;
assert_eq!(RE3_VARIATION.original_name(), "Ruy Lopez: Marshall Attack, Re3 Variation");
```"#
)]
pub static RE3_VARIATION: Variation = Variation {
    name: "Re3 Variation",
    parent: Some(&super::MARSHALL_ATTACK),
    variations: &[],
    lines: &[Line {
        parent: &RE3_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<8>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B8,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::A7,
                capture: None,
                to: Square::A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::B5,
                capture: None,
                to: Square::A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Castle {
                king: Square::E1,
                rook: Square::H1,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: Square::F1,
                capture: None,
                to: Square::E1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::B7,
                capture: None,
                to: Square::B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::A4,
                capture: None,
                to: Square::B3,
                promotion: None,
            },
            Castle {
                king: Square::E8,
                rook: Square::H8,
            },
            Normal {
                role: Pawn,
                from: Square::C2,
                capture: None,
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E4,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F6,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F3,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::C6,
                capture: Some(Knight),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: Square::E1,
                capture: Some(Knight),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::B3,
                capture: Some(Knight),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C6,
                capture: Some(Bishop),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D2,
                capture: None,
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::E7,
                capture: None,
                to: Square::D6,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: Square::E5,
                capture: None,
                to: Square::E3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(63051537379025664),
                    knight: Bitboard(2),
                    bishop: Bitboard(288239172244733956),
                    rook: Bitboard(2377900603252670465),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7917338083471654912),
                    white: Bitboard(135586639),
                },
            ) {
                board
            } else {
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(14) {
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
