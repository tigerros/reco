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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// English Opening: King's English Variation, Four Knights Variation, Quiet Line.
pub static QUIET_LINE: [Opening<&str>; 3] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<28>(),
    },
    name: Cow::Borrowed(&["English Opening", "King's English Variation", "Four Knights Variation", "Quiet Line"]),
    moves: Cow::Borrowed(&[
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
        to: E5,
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
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
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: C2,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Knight,
        from: C3,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: F8,
        capture: None,
        to: E8,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: C2,
        capture: None,
        to: F5,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588221541120),
                knight: Bitboard(39616780435456),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(1224979098644775041),
                queen: Bitboard(576460889742376960),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(6768668416132775936),
                white: Bitboard(171869006773)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 6,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}, Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<28>(),
    },
    name: Cow::Borrowed(&["English Opening", "King's English Variation", "Four Knights Variation", "Quiet Line"]),
    moves: Cow::Borrowed(&[
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
        to: E5,
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
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
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
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
        role: Bishop,
        from: B4,
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
                pawn: Bitboard(67272588221541120),
                knight: Bitboard(39582420697088),
                bishop: Bitboard(288230376151973924),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303424512),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(11380354434526871552),
                white: Bitboard(70315957)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}, Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<28>(),
    },
    name: Cow::Borrowed(&["English Opening", "King's English Variation", "Four Knights Variation", "Quiet Line"]),
    moves: Cow::Borrowed(&[
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
        to: E5,
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E3,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588221541120),
                knight: Bitboard(39582420959232),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13686197443740303360),
                white: Bitboard(70577085)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];