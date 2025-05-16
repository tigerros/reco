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
/// Scotch Game: Cochrane-Shumov Defense.
pub const COCHRANE_SHUMOV_DEFENSE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<44>(),
    },
    name: &["Scotch Game", "Cochrane-Shumov Defense"],
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
            role: Pawn,
            from: D2,
            capture: None,
            to: D4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E5,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F1,
            capture: None,
            to: C4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F8,
            capture: None,
            to: C5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F3,
            capture: None,
            to: G5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G8,
            capture: None,
            to: H6,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G5,
            capture: Some(Pawn),
            to: F7,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: H6,
            capture: Some(Knight),
            to: F7,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C4,
            capture: Some(Knight),
            to: F7,
            promotion: None,
        },
        Normal {
            role: King,
            from: E8,
            capture: Some(Bishop),
            to: F7,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: H5,
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
            role: Queen,
            from: H5,
            capture: Some(Bishop),
            to: C5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D7,
            capture: None,
            to: D5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(38069525362566912),
                knight: Bitboard(4398046511106),
                bishop: Bitboard(288230376151711748),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460769483292672),
                king: Bitboard(9007199254741008),
            },
            ByColor {
                black: Bitboard(10207201881743163392),
                white: Bitboard(17448363927),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
