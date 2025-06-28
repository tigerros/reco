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
# use reco::book::italian_game::evans_gambit_declined::PAVLOV_VARIATION;
assert_eq!(PAVLOV_VARIATION.original_name(), "Italian Game: Evans Gambit Declined, Pavlov Variation");
```"#
)]
pub static PAVLOV_VARIATION: Variation = Variation {
    name: "Pavlov Variation",
    parent: Some(&super::EVANS_GAMBIT_DECLINED),
    variations: &[],
    lines: &[Line {
        parent: &PAVLOV_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<5>(),
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
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::B2,
                capture: None,
                to: Square::B4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C5,
                capture: None,
                to: Square::B6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::B4,
                capture: None,
                to: Square::B5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::C6,
                capture: None,
                to: Square::A5,
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
                from: Square::G8,
                capture: None,
                to: Square::H6,
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
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C1,
                capture: Some(Knight),
                to: Square::H6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D6,
                capture: Some(Knight),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::H6,
                capture: Some(Pawn),
                to: Square::G7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: Square::H8,
                capture: None,
                to: Square::G8,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C4,
                capture: Some(Pawn),
                to: Square::F7,
                promotion: None,
            },
            Normal {
                role: King,
                from: Square::E8,
                capture: Some(Bishop),
                to: Square::F7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::G7,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B1,
                capture: None,
                to: Square::D2,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(37999130848584960),
                    knight: Bitboard(4294969344),
                    bishop: Bitboard(288232643894444032),
                    rook: Bitboard(4683743612465315969),
                    queen: Bitboard(274877906952),
                    king: Bitboard(9007199254741008),
                },
                ByColor {
                    black: Bitboard(5018982787923836928),
                    white: Bitboard(77712125337),
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
            castling_rights: Bitboard(129),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
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
