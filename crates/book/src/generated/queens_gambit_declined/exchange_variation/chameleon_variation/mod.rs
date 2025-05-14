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

/// Queen's Gambit Declined: Exchange Variation, Chameleon Variation.
pub const CHAMELEON_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<35>(),
    },
    name: "Queen's Gambit Declined",
    variation: &["Exchange Variation", "Chameleon Variation"],
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
            from: B1,
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
            from: C4,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E6,
            capture: Some(Pawn),
            to: D5,
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
            role: Bishop,
            from: F1,
            capture: None,
            to: D3,
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
            role: Queen,
            from: D1,
            capture: None,
            to: C2,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: F8,
            capture: None,
            to: E8,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G1,
            capture: None,
            to: E2,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: D7,
            capture: None,
            to: F8,
            promotion: None,
        },
        Castle { king: E1, rook: A1 },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65020754115224320),
                knight: Bitboard(2305878193586049024),
                bishop: Bitboard(292734250657513472),
                rook: Bitboard(1224979098644775048),
                queen: Bitboard(576460752303424512),
                king: Bitboard(4611686018427387908),
            },
            ByColor {
                black: Bitboard(9076758792720351232),
                white: Bitboard(275014023052),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 8,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
