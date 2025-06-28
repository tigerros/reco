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
# use reco::book::queens_gambit_declined::lasker_defense::RUSSIAN_VARIATION;
assert_eq!(RUSSIAN_VARIATION.original_name(), "Queen's Gambit Declined: Lasker Defense, Russian Variation");
```"#
)]
pub static RUSSIAN_VARIATION: Variation = Variation {
    name: "Russian Variation",
    parent: Some(&super::LASKER_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &RUSSIAN_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<5>(),
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
                from: Square::E7,
                capture: None,
                to: Square::E6,
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
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E3,
                promotion: None,
            },
            Castle {
                king: Square::E8,
                rook: Square::H8,
            },
            Normal {
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::H7,
                capture: None,
                to: Square::H6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::G5,
                capture: None,
                to: Square::H4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F6,
                capture: None,
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::H4,
                capture: Some(Bishop),
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: Some(Bishop),
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: None,
                to: Square::C2,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::E4,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::D3,
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
                role: Bishop,
                from: Square::D3,
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
                role: Bishop,
                from: Square::C8,
                capture: None,
                to: Square::D7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(28024369683948288),
                    knight: Bitboard(39582420959232),
                    bishop: Bitboard(2251799880794112),
                    rook: Bitboard(2377900603251621897),
                    queen: Bitboard(4503599627371520),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7024405973087289344),
                    white: Bitboard(204793673),
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
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
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
