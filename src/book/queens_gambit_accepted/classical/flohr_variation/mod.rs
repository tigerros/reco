use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
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

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Queen's Gambit Accepted: Classical, Flohr Variation.
pub static FLOHR_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<28>(),
    },
    name: Cow::Borrowed(&["Queen's Gambit Accepted", "Classical", "Flohr Variation"]),
    moves: Cow::Borrowed(&[
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
            from: D5,
            capture: Some(Pawn),
            to: C4,
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
            from: G8,
            capture: None,
            to: F6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E2,
            capture: None,
            to: E3,
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
            role: Bishop,
            from: F1,
            capture: Some(Pawn),
            to: C4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C7,
            capture: None,
            to: C5,
            promotion: None,
        },
        Castle { king: E1, rook: H1 },
        Normal {
            role: Pawn,
            from: A7,
            capture: None,
            to: A6,
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
            from: C4,
            capture: None,
            to: B3,
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
            role: Rook,
            from: F1,
            capture: None,
            to: D1,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C5,
            capture: None,
            to: C4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: B3,
            capture: None,
            to: C2,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: C6,
            capture: None,
            to: B4,
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
            from: B4,
            capture: Some(Bishop),
            to: C2,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: E2,
            capture: Some(Knight),
            to: C2,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: B7,
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
            role: Queen,
            from: D8,
            capture: None,
            to: C7,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63069129498747648),
                knight: Bitboard(35184374448128),
                bishop: Bitboard(2306405959167115268),
                rook: Bitboard(9295429630892703753),
                queen: Bitboard(1125899906843648),
                king: Bitboard(1152921504606847040),
            },
            ByColor {
                black: Bitboard(12818987274083500032),
                white: Bitboard(34363205453),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(14) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
