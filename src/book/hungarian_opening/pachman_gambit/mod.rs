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
/// Hungarian Opening: Pachman Gambit.
pub const PACHMAN_GAMBIT: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<0>(),
    },
    name: "Hungarian Opening",
    variation: &["Pachman Gambit"],
    moves: &[
        Normal {
            role: Pawn,
            from: G2,
            capture: None,
            to: G3,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: F7,
            capture: None,
            to: F5,
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
            from: F5,
            capture: Some(Pawn),
            to: E4,
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
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(44824890313846528),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576461302059237376),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18419511369981231104),
                white: Bitboard(549760053239),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
