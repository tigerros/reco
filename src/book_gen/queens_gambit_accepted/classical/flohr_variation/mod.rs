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
# use reco::book::queens_gambit_accepted::classical::FLOHR_VARIATION;
assert_eq!(FLOHR_VARIATION.original_name(), "Queen's Gambit Accepted: Classical, Flohr Variation");
```"#
)]
pub static FLOHR_VARIATION: Variation = Variation {
    name: "Flohr Variation",
    parent: Some(&super::CLASSICAL),
    variations: &[],
    lines: &[Line {
        parent: &FLOHR_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<2>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::D2,
                capture: None,
                to: Square::D4,
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
                from: Square::C2,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D5,
                capture: Some(Pawn),
                to: Square::C4,
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
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: Some(Pawn),
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C5,
                promotion: None,
            },
            Castle {
                king: Square::E1,
                rook: Square::H1,
            },
            Normal {
                role: Pawn,
                from: Square::A7,
                capture: None,
                to: Square::A6,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: None,
                to: Square::E2,
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
                from: Square::C4,
                capture: None,
                to: Square::B3,
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
                role: Rook,
                from: Square::F1,
                capture: None,
                to: Square::D1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C5,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::B3,
                capture: None,
                to: Square::C2,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::C6,
                capture: None,
                to: Square::B4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B1,
                capture: None,
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B4,
                capture: Some(Bishop),
                to: Square::C2,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::E2,
                capture: Some(Knight),
                to: Square::C2,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C8,
                capture: None,
                to: Square::B7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D4,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::C7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(63069129498747648),
                    knight: Bitboard(35184374448128),
                    bishop: Bitboard(2306405959167115268),
                    rook: Bitboard(9295429630892703753),
                    queen: Bitboard(1125899906843648),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(12818987274083500032),
                    white: Bitboard(34363205453),
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
            turn: White,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
