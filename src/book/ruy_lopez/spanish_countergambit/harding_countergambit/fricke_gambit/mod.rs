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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Ruy Lopez: Spanish Countergambit, Harding Countergambit, Fricke Gambit.
pub static FRICKE_GAMBIT: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<60>(),
    },
    name: Cow::Borrowed(&["Ruy Lopez", "Spanish Countergambit", "Harding Countergambit", "Fricke Gambit"]),
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
        role: Bishop,
        from: F1,
        capture: None,
        to: B5,
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
        from: F3,
        capture: Some(
            Pawn,
        ),
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
    Castle {
        king: E1,
        rook: H1,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65020754248396544),
                knight: Bitboard(4611690485193375746),
                bishop: Bitboard(2594073393955340292),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(274877906952),
                king: Bitboard(1152921504606847040)
            },
            ByColor {
                black: Bitboard(17719135966196662272),
                white: Bitboard(77577908079)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];