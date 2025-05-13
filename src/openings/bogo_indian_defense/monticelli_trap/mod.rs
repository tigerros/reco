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

/// Bogo-Indian Defense: Monticelli Trap.
pub const MONTICELLI_TRAP: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        subcategory: RangedU8::new_static::<11>(),
    },
    name: "Bogo-Indian Defense",
    variation: &["Monticelli Trap"],
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
            from: B7,
            capture: None,
            to: B6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: G2,
            capture: None,
            to: G3,
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
            role: Bishop,
            from: F1,
            capture: None,
            to: G2,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F8,
            capture: None,
            to: B4,
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
            role: Bishop,
            from: B4,
            capture: Some(Bishop),
            to: D2,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: Some(Bishop),
            to: D2,
            promotion: None,
        },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Knight,
            from: B1,
            capture: None,
            to: C3,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F6,
            capture: None,
            to: E4,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D2,
            capture: None,
            to: C2,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: E4,
            capture: Some(Knight),
            to: C3,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F3,
            capture: None,
            to: G5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66729360895292160),
                knight: Bitboard(144115462954024960),
                bishop: Bitboard(562949953437696),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303424512),
                king: Bitboard(4611686018427387920),
            },
            ByColor {
                black: Bitboard(7777454872701698048),
                white: Bitboard(275083491217),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
