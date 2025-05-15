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

/// Italian Game: Evans Gambit Declined, Pavlov Variation.
pub const PAVLOV_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<51>(),
    },
    name: "Italian Game",
    variation: &["Evans Gambit Declined", "Pavlov Variation"],
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
            role: Pawn,
            from: B2,
            capture: None,
            to: B4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C5,
            capture: None,
            to: B6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: B4,
            capture: None,
            to: B5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: C6,
            capture: None,
            to: A5,
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
            from: G8,
            capture: None,
            to: H6,
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
            to: D6,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C1,
            capture: Some(Knight),
            to: H6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D6,
            capture: Some(Knight),
            to: E5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: H6,
            capture: Some(Pawn),
            to: G7,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: H8,
            capture: None,
            to: G8,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C4,
            capture: Some(Pawn),
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
            role: Bishop,
            from: G7,
            capture: Some(Pawn),
            to: E5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: G5,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: B1,
            capture: None,
            to: D2,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(37999130848584960),
                knight: Bitboard(4294969344),
                bishop: Bitboard(288232643894444032),
                rook: Bitboard(4683743612465315969),
                queen: Bitboard(274877906952),
                king: Bitboard(9007199254741008),
            },
            ByColor {
                black: Bitboard(5018982787923836928),
                white: Bitboard(77712125337),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
