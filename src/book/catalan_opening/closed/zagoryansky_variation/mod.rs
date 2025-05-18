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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Catalan Opening: Closed, Zagoryansky Variation.
pub static ZAGORYANSKY_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<8>(),
    },
    name: Cow::Borrowed(&["Catalan Opening", "Closed", "Zagoryansky Variation"]),
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
        from: E7,
        capture: None,
        to: E6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
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
        from: G1,
        capture: None,
        to: F3,
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
        from: G2,
        capture: None,
        to: G3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: G2,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Knight,
        from: B8,
        capture: None,
        to: D7,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: C2,
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
        role: Rook,
        from: F1,
        capture: None,
        to: D1,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: None,
        to: B6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: A2,
        capture: None,
        to: A4,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63356093597790720),
                knight: Bitboard(2286984187871234),
                bishop: Bitboard(292733975779098628),
                rook: Bitboard(2377900603251621897),
                queen: Bitboard(576460752303424512),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7924424427322736640),
                white: Bitboard(224458319)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];