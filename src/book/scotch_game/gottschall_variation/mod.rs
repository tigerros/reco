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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Scotch Game: Gottschall Variation.
pub static GOTTSCHALL_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<45>(),
    },
    name: Cow::Borrowed(&["Scotch Game", "Gottschall Variation"]),
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
        role: Bishop,
        from: F8,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: E3,
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
        role: Pawn,
        from: C2,
        capture: None,
        to: C3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: D2,
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
        from: D4,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C5,
        capture: Some(
            Bishop,
        ),
        to: E3,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D2,
        capture: Some(
            Bishop,
        ),
        to: E3,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Knight,
        from: B5,
        capture: Some(
            Pawn,
        ),
        to: C7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: A8,
        capture: None,
        to: B8,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: C7,
        capture: Some(
            Pawn,
        ),
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: E7,
        capture: Some(
            Knight,
        ),
        to: D5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E4,
        capture: Some(
            Knight,
        ),
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: C6,
        capture: None,
        to: B4,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63894854073377536),
                knight: Bitboard(33554434),
                bishop: Bitboard(288230376151711776),
                rook: Bitboard(2449958197289549953),
                queen: Bitboard(35184373137408),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(7413804595987611648),
                white: Bitboard(34361107379)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];