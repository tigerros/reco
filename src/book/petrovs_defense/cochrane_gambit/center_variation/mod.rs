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
/// Petrov's Defense: Cochrane Gambit, Center Variation.
pub const CENTER_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<42>(),
    },
    name: "Petrov's Defense",
    variation: &["Cochrane Gambit", "Center Variation"],
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
            from: G8,
            capture: None,
            to: F6,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F3,
            capture: Some(Pawn),
            to: E5,
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
            from: E5,
            capture: Some(Pawn),
            to: F7,
            promotion: None,
        },
        Normal {
            role: King,
            from: E8,
            capture: Some(Knight),
            to: F7,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D2,
            capture: None,
            to: D4,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(56022316861155072),
                knight: Bitboard(144150372447944706),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(9007199254741008),
            },
            ByColor {
                black: Bitboard(12675143656722661376),
                white: Bitboard(402712511),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
