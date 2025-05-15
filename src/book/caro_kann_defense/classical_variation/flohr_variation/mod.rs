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
/// Caro-Kann Defense: Classical Variation, Flohr Variation.
pub const FLOHR_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::B,
        category: RangedU8::new_static::<18>(),
    },
    name: "Caro-Kann Defense",
    variation: &["Classical Variation", "Flohr Variation"],
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
            to: C6,
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
            from: D7,
            capture: None,
            to: D5,
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
            from: D5,
            capture: Some(Pawn),
            to: E4,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: D2,
            capture: Some(Pawn),
            to: E4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: F5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: E4,
            capture: None,
            to: G3,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F5,
            capture: None,
            to: G6,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G1,
            capture: None,
            to: H3,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68402817521477376),
                knight: Bitboard(4755801206515826688),
                bishop: Bitboard(2305913377957871652),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18154929289651290112),
                white: Bitboard(146859965),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 4,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
