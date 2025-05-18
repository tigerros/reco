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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// French Defense: Tarrasch Variation, Pawn Center Variation.
pub static PAWN_CENTER_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<5>(),
    },
    name: Cow::Borrowed(&["French Defense", "Tarrasch Variation", "Pawn Center Variation"]),
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
        to: E6,
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
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B1,
        capture: None,
        to: D2,
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
        from: E4,
        capture: None,
        to: E5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: None,
        to: D7,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65038415556560640),
                knight: Bitboard(146366987889543232),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13830290607223865344),
                white: Bitboard(69390618621)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];