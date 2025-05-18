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
/// Polish Opening: Rooks Swap Line.
pub static ROOKS_SWAP_LINE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<0>(),
    },
    name: Cow::Borrowed(&["Polish Opening", "Rooks Swap Line"]),
    moves: Cow::Borrowed(&[
        Normal {
            role: Pawn,
            from: B2,
            capture: None,
            to: B4,
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
            role: Bishop,
            from: C1,
            capture: None,
            to: B2,
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
            from: B4,
            capture: None,
            to: B5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: A7,
            capture: None,
            to: A6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: A2,
            capture: None,
            to: A4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: A6,
            capture: Some(Pawn),
            to: B5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: A4,
            capture: Some(Pawn),
            to: B5,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: A8,
            capture: Some(Rook),
            to: A1,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: B2,
            capture: Some(Rook),
            to: A1,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67008645233179648),
                knight: Bitboard(144150372447944770),
                bishop: Bitboard(2594073385365405729),
                rook: Bitboard(9223372036854775936),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13757986688221577216),
                white: Bitboard(8589999355),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9223372036854775936),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
