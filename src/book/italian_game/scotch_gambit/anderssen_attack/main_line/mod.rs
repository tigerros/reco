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
/// Italian Game: Scotch Gambit, Anderssen Attack, Main Line.
pub const MAIN_LINE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<56>(),
    },
    name: &[
        "Italian Game",
        "Scotch Gambit",
        "Anderssen Attack",
        "Main Line",
    ],
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
            role: Knight,
            from: G8,
            capture: None,
            to: F6,
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
            role: Knight,
            from: F6,
            capture: Some(Pawn),
            to: E4,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: F1,
            capture: None,
            to: E1,
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
            role: Bishop,
            from: C4,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: Some(Bishop),
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
            role: Queen,
            from: D5,
            capture: None,
            to: A5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: C3,
            capture: Some(Knight),
            to: E4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: E6,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C1,
            capture: None,
            to: D2,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: A5,
            capture: None,
            to: D5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: D2,
            capture: None,
            to: G5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65020719754438400),
                knight: Bitboard(4398317043712),
                bishop: Bitboard(2305860876277645312),
                rook: Bitboard(9295429630892703761),
                queen: Bitboard(34359738376),
                king: Bitboard(1152921504606847040),
            },
            ByColor {
                black: Bitboard(12819236889059917824),
                white: Bitboard(275148498777),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 4,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
