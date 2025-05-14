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

/// Queen's Gambit Declined: Neo-Orthodox Variation, Main Line.
pub const MAIN_LINE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<55>(),
    },
    name: "Queen's Gambit Declined",
    variation: &["Neo-Orthodox Variation", "Main Line"],
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
            to: E7,
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
        Castle { king: E8, rook: H8 },
        Normal {
            role: Pawn,
            from: E2,
            capture: None,
            to: E3,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(29150286837768960),
                knight: Bitboard(144150372450304000),
                bishop: Bitboard(292733977926565920),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920),
            },
            ByColor {
                black: Bitboard(8032082008844795904),
                white: Bitboard(2352276409),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
