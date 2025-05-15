use crate::{Code, Opening, Volume};
use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
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
use shakmaty::Square::*;
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByColor, ByRole, Setup};

#[allow(clippy::doc_markdown)]
/// Queen's Gambit Declined: Orthodox Defense, Main Line.
pub const MAIN_LINE: [Opening<'static, &str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<63>(),
        },
        name: "Queen's Gambit Declined",
        variation: &["Orthodox Defense", "Main Line"],
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
                from: E7,
                capture: None,
                to: E6,
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
                from: D7,
                capture: None,
                to: D5,
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
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: D7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: A1,
                capture: None,
                to: C1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C7,
                capture: None,
                to: C6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63916844508046080),
                    knight: Bitboard(2286984188133376),
                    bishop: Bitboard(292734250656989216),
                    rook: Bitboard(2377900603251622020),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(7924985178252902400),
                    white: Bitboard(275082699708),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(128),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<63>(),
        },
        name: "Queen's Gambit Declined",
        variation: &["Orthodox Defense", "Main Line"],
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
                from: E7,
                capture: None,
                to: E6,
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
                from: D7,
                capture: None,
                to: D5,
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
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: D7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: A1,
                capture: None,
                to: C1,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346368377600),
                    knight: Bitboard(2286984188133376),
                    bishop: Bitboard(292734250656989216),
                    rook: Bitboard(2377900603251622020),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(7926106680113233920),
                    white: Bitboard(275082699708),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(128),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<67>(),
        },
        name: "Queen's Gambit Declined",
        variation: &["Orthodox Defense", "Main Line"],
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
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
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
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
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
                to: D7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: A1,
                capture: None,
                to: C1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C7,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F1,
                capture: None,
                to: D3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D5,
                capture: Some(Pawn),
                to: C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: D3,
                capture: Some(Pawn),
                to: C4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: G5,
                capture: Some(Bishop),
                to: E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: Some(Bishop),
                to: E7,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63916810081198848),
                    knight: Bitboard(2251834175782912),
                    bishop: Bitboard(288230376218820608),
                    rook: Bitboard(2377900603251621924),
                    queen: Bitboard(4503599627370504),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7348489241577390080),
                    white: Bitboard(204792684),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
