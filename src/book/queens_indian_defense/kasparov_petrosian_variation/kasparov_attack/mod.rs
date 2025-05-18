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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Queen's Indian Defense: Kasparov-Petrosian Variation, Kasparov Attack.
pub static KASPAROV_ATTACK: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<12>(),
    },
    name: Cow::Borrowed(&["Queen's Indian Defense", "Kasparov-Petrosian Variation", "Kasparov Attack"]),
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
        from: E7,
        capture: None,
        to: E6,
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
        from: B7,
        capture: None,
        to: B6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: A2,
        capture: None,
        to: A3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C8,
        capture: None,
        to: B7,
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
        role: Queen,
        from: D1,
        capture: None,
        to: C2,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(64477561010385408),
                knight: Bitboard(144115222437953536),
                bishop: Bitboard(2306405959167115300),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303424512),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13539810630281723904),
                white: Bitboard(136705717)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];