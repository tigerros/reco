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

/// King's Indian Defense: Petrosian Variation, Keres Defense.
pub const KERES_DEFENSE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<93>(),
    },
    name: "King's Indian Defense",
    variation: &["Petrosian Variation", "Keres Defense"],
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
            from: G7,
            capture: None,
            to: G6,
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
            role: Bishop,
            from: F8,
            capture: None,
            to: G7,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E2,
            capture: None,
            to: E4,
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
            from: G1,
            capture: None,
            to: F3,
            promotion: None,
        },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Bishop,
            from: F1,
            capture: None,
            to: E2,
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
            from: D4,
            capture: None,
            to: D5,
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
            from: C1,
            capture: None,
            to: G5,
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
            role: Bishop,
            from: G5,
            capture: None,
            to: H4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: G6,
            capture: None,
            to: G5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: H4,
            capture: None,
            to: G3,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F6,
            capture: None,
            to: H5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: H2,
            capture: None,
            to: H4,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(11127438113268480),
                knight: Bitboard(2252349571858432),
                bishop: Bitboard(306244774665392128),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920),
            },
            ByColor {
                black: Bitboard(7885671899483602944),
                white: Bitboard(36849349529),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
