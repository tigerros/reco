use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Color::{Black, White};
#[allow(
    unused_imports,
    clippy::enum_glob_use,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Move::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};
#[allow(
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
use shakmaty::Square::*;
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByColor, ByRole, Setup};

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Sicilian Defense: Wing Gambit, Marienbad Variation.
pub static MARIENBAD_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::B,
        category: RangedU8::new_static::<20>(),
    },
    name: Cow::Borrowed(&["Sicilian Defense", "Wing Gambit", "Marienbad Variation"]),
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
            role: Pawn,
            from: B2,
            capture: None,
            to: B4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C5,
            capture: Some(Pawn),
            to: B4,
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
            role: Pawn,
            from: D7,
            capture: None,
            to: D5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E4,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C1,
            capture: None,
            to: B2,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68398419374369792),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365406240),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(34359738376),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(17866624181102182400),
                white: Bitboard(126715),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
