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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Rubinstein Opening: Bogoljubow Defense.
pub static BOGOLJUBOW_DEFENSE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<5>(),
    },
    name: Cow::Borrowed(&["Rubinstein Opening", "Bogoljubow Defense"]),
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
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
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
        from: E2,
        capture: None,
        to: E3,
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
        role: Bishop,
        from: F1,
        capture: None,
        to: D3,
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
        from: B2,
        capture: None,
        to: B3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: B2,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912463574426880),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(288239172245258752),
                rook: Bitboard(2377900603251621921),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7918238592084738048),
                white: Bitboard(138078059)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 5,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];