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
/// Italian Game: Evans Gambit, Mortimer-Evans Gambit
pub static MORTIMER_EVANS_GAMBIT: Variation = Variation {
    name: "Mortimer-Evans Gambit",
    parent: Some(&super::EVANS_GAMBIT),
    variations: &[],
    lines: &[Line {
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<51>(),
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
                to: C5,
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
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C3,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: None,
                to: B6,
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
                role: Bishop,
                from: C8,
                capture: None,
                to: G4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: G4,
                capture: None,
                to: D7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: A4,
                capture: None,
                to: B3,
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
                role: Bishop,
                from: C4,
                capture: Some(Pawn),
                to: F7,
                promotion: None,
            },
            Normal {
                role: King,
                from: E8,
                capture: None,
                to: F8,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: B3,
                capture: None,
                to: C2,
                promotion: None,
            },
            Normal {
                role: King,
                from: F8,
                capture: Some(Bishop),
                to: F7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(56022316861153536),
                    knight: Bitboard(4611686022724714496),
                    bishop: Bitboard(2253998836940804),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(576460752303424512),
                    king: Bitboard(9007199254741056),
                },
                ByColor {
                    black: Bitboard(14550859920468606976),
                    white: Bitboard(405071205),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
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
