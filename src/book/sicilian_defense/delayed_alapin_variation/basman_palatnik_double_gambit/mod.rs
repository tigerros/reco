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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Sicilian Defense: Delayed Alapin Variation, Basman-Palatnik Double Gambit.
pub static BASMAN_PALATNIK_DOUBLE_GAMBIT: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::B,
        category: RangedU8::new_static::<50>(),
    },
    name: Cow::Borrowed(&["Sicilian Defense", "Delayed Alapin Variation", "Basman-Palatnik Double Gambit"]),
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
        from: C7,
        capture: None,
        to: C5,
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
        to: D6,
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
        to: F6,
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
        from: C5,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C3,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: Some(
            Pawn,
        ),
        to: E4,
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
        role: Queen,
        from: D8,
        capture: None,
        to: A5,
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
        from: E4,
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
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68407249793769728),
                knight: Bitboard(4398048608256),
                bishop: Bitboard(2594073385365409796),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(4294967304),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13110836138640146432),
                white: Bitboard(34362159517)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];