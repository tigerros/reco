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
/// Sicilian Defense: Dragon Variation, Classical Variation, Battery Variation.
pub const BATTERY_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::B,
        category: RangedU8::new_static::<73>(),
    },
    name: "Sicilian Defense",
    variation: &[
        "Dragon Variation",
        "Classical Variation",
        "Battery Variation",
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
            from: C7,
            capture: None,
            to: C5,
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
            role: Pawn,
            from: D2,
            capture: None,
            to: D4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C5,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F3,
            capture: Some(Pawn),
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
            role: Knight,
            from: B1,
            capture: None,
            to: C3,
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
            role: Bishop,
            from: F1,
            capture: None,
            to: E2,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F8,
            capture: None,
            to: G7,
            promotion: None,
        },
        Castle { king: E1, rook: H1 },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Bishop,
            from: C1,
            capture: None,
            to: E3,
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
            role: Queen,
            from: D1,
            capture: None,
            to: D2,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(50463185936901888),
                knight: Bitboard(39582553079808),
                bishop: Bitboard(306244774662246400),
                rook: Bitboard(2377900603251621921),
                queen: Bitboard(576460752303425536),
                king: Bitboard(4611686018427387968),
            },
            ByColor {
                black: Bitboard(7922794916730634240),
                white: Bitboard(404029281),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 7,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
