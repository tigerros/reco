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
/// King's Gambit Accepted: Greco Gambit.
pub const GRECO_GAMBIT: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<34>(),
        },
        name: "King's Gambit Accepted",
        variation: &["Greco Gambit"],
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
                role: Pawn,
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: F4,
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
                to: D6,
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
                role: Pawn,
                from: H7,
                capture: None,
                to: H6,
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
                from: G7,
                capture: None,
                to: G5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H2,
                capture: None,
                to: H4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: G7,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(11127335638025984),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(306244774728302596),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16097985202055282688),
                    white: Bitboard(2619361183),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<38>(),
        },
        name: "King's Gambit Accepted",
        variation: &["Greco Gambit"],
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
                role: Pawn,
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: F4,
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
                from: G7,
                capture: None,
                to: G5,
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
                to: G7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H2,
                capture: None,
                to: H4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H7,
                capture: None,
                to: H6,
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
                from: D7,
                capture: None,
                to: D6,
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
                from: C7,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H4,
                capture: Some(Pawn),
                to: G5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H6,
                capture: Some(Pawn),
                to: G5,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: H1,
                capture: Some(Rook),
                to: H8,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: G7,
                capture: Some(Rook),
                to: H8,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: E5,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(9865094141855488),
                    knight: Bitboard(4755801275222982656),
                    bishop: Bitboard(9511602413073596420),
                    rook: Bitboard(72057594037927937),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16078708564197113856),
                    white: Bitboard(69189519133),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(72057594037927937),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
