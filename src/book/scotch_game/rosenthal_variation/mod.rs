#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use alloc::borrow::Cow;
use deranged::RangedU8;
use core::panic;

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Scotch Game: Rosenthal Variation.
pub static ROSENTHAL_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<45>(),
    },
    name: Cow::Borrowed(&["Scotch Game", "Rosenthal Variation"]),
    moves: Cow::Borrowed(&[
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
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F3,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: H4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: D4,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: D2,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: H4,
        capture: Some(
            Pawn,
        ),
        to: E4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: E2,
        promotion: None,
    },
    Normal {
        role: King,
        from: E8,
        capture: None,
        to: D8,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: B4,
        capture: Some(
            Bishop,
        ),
        to: D2,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B1,
        capture: Some(
            Bishop,
        ),
        to: D2,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: E4,
        capture: None,
        to: G6,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272519433905920),
                knight: Bitboard(4611690425063835648),
                bishop: Bitboard(288230376151715840),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(70368744177672),
                king: Bitboard(576460752303423552)
            },
            ByColor {
                black: Bitboard(14839154063999762432),
                white: Bitboard(8589999977)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];