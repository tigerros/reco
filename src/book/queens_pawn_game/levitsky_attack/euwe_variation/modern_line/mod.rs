use crate::{Code, Opening, Volume};
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

#[allow(clippy::doc_markdown)]
/// Queen's Pawn Game: Levitsky Attack, Euwe Variation, Modern Line.
pub const MODERN_LINE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<2>(),
    },
    name: "Queen's Pawn Game",
    variation: &["Levitsky Attack", "Euwe Variation", "Modern Line"],
    moves: &[
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
            from: G1,
            capture: None,
            to: F3,
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
            role: Bishop,
            from: C1,
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
            role: Bishop,
            from: G5,
            capture: None,
            to: H4,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: B6,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(32514792350611200),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(2594073387512889376),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(2199023255560),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(17830742718607785984),
                white: Bitboard(2283861947),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
