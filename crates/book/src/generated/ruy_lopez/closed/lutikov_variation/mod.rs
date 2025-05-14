use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
use reco_core::{Code, Opening, Volume};
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

/// Ruy Lopez: Closed, Lutikov Variation.
pub const LUTIKOV_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<90>(),
    },
    name: "Ruy Lopez",
    variation: &["Closed", "Lutikov Variation"],
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
            role: Rook,
            from: F1,
            capture: None,
            to: E1,
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
        Normal {
            role: Pawn,
            from: D7,
            capture: None,
            to: D6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C2,
            capture: None,
            to: C3,
            promotion: None,
        },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Bishop,
            from: B3,
            capture: None,
            to: C2,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(64186267872848640),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975779083268),
                rook: Bitboard(2377900603251621905),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968),
            },
            ByColor {
                black: Bitboard(7923007199784206336),
                white: Bitboard(270856031),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
