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
/// King's Indian Defense: Sämisch Variation, Double Fianchetto.
pub const DOUBLE_FIANCHETTO: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<82>(),
    },
    name: "King's Indian Defense",
    variation: &["Sämisch Variation", "Double Fianchetto"],
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
            role: Pawn,
            from: F2,
            capture: None,
            to: F3,
            promotion: None,
        },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Bishop,
            from: C1,
            capture: None,
            to: E3,
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
                pawn: Bitboard(51028335116993280),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(306244774662242336),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920),
            },
            ByColor {
                black: Bitboard(8067470855736655872),
                white: Bitboard(473220089),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
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
