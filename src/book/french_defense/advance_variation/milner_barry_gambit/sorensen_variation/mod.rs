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
/// French Defense: Advance Variation, Milner-Barry Gambit, Sørensen Variation.
pub const SORENSEN_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<2>(),
    },
    name: "French Defense",
    variation: &[
        "Advance Variation",
        "Milner-Barry Gambit",
        "Sørensen Variation",
    ],
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
            to: E6,
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
            role: Pawn,
            from: E4,
            capture: None,
            to: E5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C7,
            capture: None,
            to: C5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C2,
            capture: None,
            to: C3,
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
            from: G1,
            capture: None,
            to: F3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: B6,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F1,
            capture: None,
            to: D3,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C5,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C3,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: D7,
            promotion: None,
        },
        Castle { king: E1, rook: H1 },
        Normal {
            role: Knight,
            from: C6,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: F3,
            capture: None,
            to: G5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912514978636544),
                knight: Bitboard(4611686293439512578),
                bishop: Bitboard(2308094809027903492),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(2199023255560),
                king: Bitboard(1152921504606847040),
            },
            ByColor {
                black: Bitboard(17432046608370892800),
                white: Bitboard(343597966191),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
