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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// King's Gambit Accepted: Allgaier, Horny Defense.
pub static HORNY_DEFENSE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<39>(),
    },
    name: Cow::Borrowed(&["King's Gambit Accepted", "Allgaier", "Horny Defense"]),
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
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E5,
        capture: Some(
            Pawn,
        ),
        to: F4,
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
        from: G7,
        capture: None,
        to: G5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: H2,
        capture: None,
        to: H4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G5,
        capture: None,
        to: G4,
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
        from: H7,
        capture: None,
        to: H6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G5,
        capture: Some(
            Pawn,
        ),
        to: F7,
        promotion: None,
    },
    Normal {
        role: King,
        from: E8,
        capture: Some(
            Knight,
        ),
        to: F7,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: Some(
            Pawn,
        ),
        to: G4,
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
        role: Queen,
        from: G4,
        capture: Some(
            Pawn,
        ),
        to: F4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: D6,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(4362864554954496),
                knight: Bitboard(144150372447944706),
                bishop: Bitboard(288239172244733988),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752840294400),
                king: Bitboard(9007199254741008)
            },
            ByColor {
                black: Bitboard(10317649989282562048),
                white: Bitboard(2952810423)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];