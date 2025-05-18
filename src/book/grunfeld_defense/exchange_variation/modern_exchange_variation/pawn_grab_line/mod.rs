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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Grünfeld Defense: Exchange Variation, Modern Exchange Variation, Pawn Grab Line.
pub static PAWN_GRAB_LINE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<85>(),
    },
    name: Cow::Borrowed(&["Grünfeld Defense", "Exchange Variation", "Modern Exchange Variation", "Pawn Grab Line"]),
    moves: Cow::Borrowed(&[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
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
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G7,
        capture: None,
        to: G6,
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
        role: Pawn,
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C4,
        capture: Some(
            Pawn,
        ),
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: Some(
            Pawn,
        ),
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
        role: Knight,
        from: D5,
        capture: Some(
            Knight,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B2,
        capture: Some(
            Knight,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: G7,
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
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: A1,
        capture: None,
        to: B1,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: E2,
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
        from: D4,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: G7,
        capture: Some(
            Pawn,
        ),
        to: C3,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(50454441383485696),
                knight: Bitboard(4398048608256),
                bishop: Bitboard(288230376151977988),
                rook: Bitboard(2377900603251622018),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(7904736554936172544),
                white: Bitboard(34630332830)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(128),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];