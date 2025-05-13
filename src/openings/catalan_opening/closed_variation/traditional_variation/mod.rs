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

/// Catalan Opening: Closed Variation, Traditional Variation.
pub const TRADITIONAL_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        subcategory: RangedU8::new_static::<9>(),
    },
    name: "Catalan Opening",
    variation: &["Closed Variation", "Traditional Variation"],
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
            role: Pawn,
            from: G2,
            capture: None,
            to: G3,
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
            from: F1,
            capture: None,
            to: G2,
            promotion: None,
        },
        Castle { king: E8, rook: H8 },
        Castle { king: E1, rook: H1 },
        Normal {
            role: Knight,
            from: B8,
            capture: None,
            to: D7,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: C2,
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
            to: D2,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: B7,
            capture: None,
            to: B6,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63356093581013760),
                knight: Bitboard(2286984187873280),
                bishop: Bitboard(292733975779098628),
                rook: Bitboard(2377900603251621921),
                queen: Bitboard(576460752303424512),
                king: Bitboard(4611686018427387968),
            },
            ByColor {
                black: Bitboard(7924424427322736640),
                white: Bitboard(207683429),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
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
}];
