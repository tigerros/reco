use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
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

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Blackmar-Diemer Gambit Accepted: Teichmann Defense, Seidel-Hall Attack.
pub static SEIDEL_HALL_ATTACK: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<0>(),
    },
    name: Cow::Borrowed(&[
        "Blackmar-Diemer Gambit Accepted",
        "Teichmann Defense",
        "Seidel-Hall Attack",
    ]),
    moves: Cow::Borrowed(&[
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
            from: E2,
            capture: None,
            to: E4,
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
            role: Pawn,
            from: F2,
            capture: None,
            to: F3,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E4,
            capture: Some(Pawn),
            to: F3,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G1,
            capture: Some(Pawn),
            to: F3,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: G4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: H2,
            capture: None,
            to: H3,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: G4,
            capture: Some(Knight),
            to: F3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: Some(Bishop),
            to: F3,
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
            from: G2,
            capture: None,
            to: G4,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68402818603550464),
                knight: Bitboard(144150372448206848),
                bishop: Bitboard(2305843009213693988),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752305520640),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13543208086851813376),
                white: Bitboard(1218709429),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
