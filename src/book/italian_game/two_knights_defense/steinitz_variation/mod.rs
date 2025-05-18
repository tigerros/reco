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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Italian Game: Two Knights Defense, Steinitz Variation.
pub static STEINITZ_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<59>(),
    },
    name: Cow::Borrowed(&["Italian Game", "Two Knights Defense", "Steinitz Variation"]),
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
        from: F3,
        capture: None,
        to: G5,
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
        capture: Some(
            Pawn,
        ),
        to: D5,
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
        role: Bishop,
        from: C4,
        capture: None,
        to: B5,
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
        from: D5,
        capture: Some(
            Pawn,
        ),
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: Some(
            Pawn,
        ),
        to: C6,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: B5,
        capture: None,
        to: E2,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: H7,
        capture: None,
        to: H6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G5,
        capture: None,
        to: H3,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(27448276995337984),
                knight: Bitboard(35188675444738),
                bishop: Bitboard(2594073385365409796),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13646368738830712832),
                white: Bitboard(8454047)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];