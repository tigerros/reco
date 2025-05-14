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

/// Four Knights Game: Halloween Gambit, Plasma Variation.
pub const PLASMA_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<47>(),
    },
    name: "Four Knights Game",
    variation: &["Halloween Gambit", "Plasma Variation"],
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
            role: Knight,
            from: B1,
            capture: None,
            to: C3,
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
            role: Knight,
            from: C6,
            capture: Some(Knight),
            to: E5,
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
            role: Knight,
            from: E5,
            capture: None,
            to: C6,
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
            from: C6,
            capture: None,
            to: E5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: F2,
            capture: None,
            to: F4,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: E5,
            capture: None,
            to: G6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E4,
            capture: None,
            to: E5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F6,
            capture: None,
            to: G8,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D5,
            capture: None,
            to: D6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C7,
            capture: Some(Pawn),
            to: D6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E5,
            capture: Some(Pawn),
            to: D6,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: F6,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: C3,
            capture: None,
            to: B5,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: A8,
            capture: None,
            to: B8,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66155416156948224),
                knight: Bitboard(4611756395761500160),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9367487224930631809),
                queen: Bitboard(35184372088840),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(17792420305973542912),
                white: Bitboard(8805219878845),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9223372036854775937),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 3,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
