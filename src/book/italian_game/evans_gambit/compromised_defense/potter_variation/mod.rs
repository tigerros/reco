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
/// Italian Game: Evans Gambit, Compromised Defense, Potter Variation
pub static POTTER_VARIATION: Variation = Variation {
    name: "Potter Variation",
    parent: Some(&super::COMPROMISED_DEFENSE),
    variations: &[],
    lines: &[Line {
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<52>(),
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
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B2,
                capture: None,
                to: B4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: Some(Pawn),
                to: B4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B4,
                capture: None,
                to: A5,
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
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Pawn,
                from: D4,
                capture: Some(Pawn),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: B3,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
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
                role: Queen,
                from: F6,
                capture: None,
                to: G6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B1,
                capture: Some(Pawn),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F1,
                capture: None,
                to: D1,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588153381120),
                    knight: Bitboard(4507997676240896),
                    bishop: Bitboard(288230380513787908),
                    rook: Bitboard(9295429630892703753),
                    queen: Bitboard(70368744308736),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(10808432401798135808),
                    white: Bitboard(68789133645),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
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
