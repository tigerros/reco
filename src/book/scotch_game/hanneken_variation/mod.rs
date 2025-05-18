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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Scotch Game: Hanneken Variation.
pub static HANNEKEN_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<44>(),
    },
    name: Cow::Borrowed(&["Scotch Game", "Hanneken Variation"]),
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
        to: B4,
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
        role: Pawn,
        from: D4,
        capture: Some(
            Pawn,
        ),
        to: C3,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Pawn,
        from: C3,
        capture: Some(
            Pawn,
        ),
        to: B2,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: Some(
            Pawn,
        ),
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
        role: Knight,
        from: F3,
        capture: None,
        to: G5,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
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
        from: C6,
        capture: Some(
            Pawn,
        ),
        to: E5,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272519433904384),
                knight: Bitboard(35527969472514),
                bishop: Bitboard(288230376252375552),
                rook: Bitboard(2377900603251621921),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7921585522693111808),
                white: Bitboard(274945074027)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];