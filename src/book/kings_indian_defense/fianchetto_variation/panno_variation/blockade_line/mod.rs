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
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// King's Indian Defense: Fianchetto Variation, Panno Variation, Blockade Line
pub static BLOCKADE_LINE: Variation = Variation {
    name: "Blockade Line",
    parent: Some(&super::PANNO_VARIATION),
    variations: &[],
    lines: &[Line {
        code: Code {
            volume: Volume::E,
            category: Category::new_static::<63>(),
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
                from: G7,
                capture: None,
                to: G6,
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
                role: Bishop,
                from: F8,
                capture: None,
                to: G7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: G2,
                capture: None,
                to: G3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Bishop,
                from: F1,
                capture: None,
                to: G2,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: C6,
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
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D4,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: C6,
                capture: None,
                to: A5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: D2,
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
                role: Queen,
                from: D1,
                capture: None,
                to: C2,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(45679330906387200),
                    knight: Bitboard(35188667320320),
                    bishop: Bitboard(306244774661210116),
                    rook: Bitboard(2377900603251621921),
                    queen: Bitboard(576460752303424512),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7918006633785982976),
                    white: Bitboard(34431369061),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: Some(E6),
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) {
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
