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
/// Semi-Slav Defense: Meran Variation, Wade Variation, Larsen Variation.
pub const LARSEN_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<47>(),
    },
    name: "Semi-Slav Defense",
    variation: &["Meran Variation", "Wade Variation", "Larsen Variation"],
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
            from: C7,
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
            to: D3,
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
            from: E3,
            capture: None,
            to: E4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: B5,
            capture: None,
            to: B4,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: C3,
            capture: None,
            to: A4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C6,
            capture: None,
            to: C5,
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
            role: Knight,
            from: F6,
            capture: None,
            to: D5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63349548013118208),
                knight: Bitboard(2251834192297984),
                bishop: Bitboard(2306405959167639556),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13396819160302878720),
                white: Bitboard(68873151389),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
