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

/// Ruy Lopez: Closed, Worrall Attack, Castling Line.
pub const CASTLING_LINE: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<86>(),
        },
        name: "Ruy Lopez",
        variation: &["Closed", "Worrall Attack", "Castling Line"],
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
                to: B5,
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
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: E2,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(66429271593250560),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(2377900603251621921),
                    queen: Bitboard(576460752303427584),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7925250203504869376),
                    white: Bitboard(270729063),
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
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<86>(),
        },
        name: "Ruy Lopez",
        variation: &["Closed", "Worrall Attack", "Castling Line"],
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
                to: B5,
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
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
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
                role: Queen,
                from: D1,
                capture: None,
                to: E2,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C3,
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
                from: D2,
                capture: None,
                to: D3,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(64177506140087040),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(2377900603251621921),
                    queen: Bitboard(576460752303427584),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7922998438050922496),
                    white: Bitboard(271512423),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
