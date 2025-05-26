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
/// King's Pawn Game: Damiano Defense, Damiano Gambit
pub static DAMIANO_GAMBIT: Variation = Variation {
    name: "Damiano Gambit",
    parent: Some(&super::DAMIANO_DEFENSE),
    variations: &[&CHIGORIN_GAMBIT],
    lines: &[Line {
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<40>(),
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
                role: Pawn,
                from: F7,
                capture: None,
                to: F6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
                to: E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: F6,
                capture: Some(Knight),
                to: E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: H5,
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
                role: Queen,
                from: H5,
                capture: Some(Pawn),
                to: E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: E5,
                capture: Some(Rook),
                to: H8,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(40321290682298112),
                    knight: Bitboard(4755801206503243778),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(72057594037928065),
                    queen: Bitboard(9227875636482146304),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(8619678580554596352),
                    white: Bitboard(9223372037123272631),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(72057594037928065),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
pub mod chigorin_gambit;
pub use chigorin_gambit::CHIGORIN_GAMBIT;
