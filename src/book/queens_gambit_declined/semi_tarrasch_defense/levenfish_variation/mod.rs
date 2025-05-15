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
/// Queen's Gambit Declined: Semi-Tarrasch Defense, Levenfish Variation.
pub const LEVENFISH_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<40>(),
    },
    name: "Queen's Gambit Declined",
    variation: &["Semi-Tarrasch Defense", "Levenfish Variation"],
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
            role: Pawn,
            from: C7,
            capture: None,
            to: C5,
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
            to: D3,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: F8,
            capture: None,
            to: D6,
            promotion: None,
        },
        Castle { king: E1, rook: H1 },
        Castle { king: E8, rook: H8 },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: E2,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: E7,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D4,
            capture: Some(Pawn),
            to: C5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: D6,
            capture: Some(Pawn),
            to: C5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E3,
            capture: None,
            to: E4,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912446594704128),
                knight: Bitboard(39582420959232),
                bishop: Bitboard(288230393332105220),
                rook: Bitboard(2377900603251621921),
                queen: Bitboard(4503599627374592),
                king: Bitboard(4611686018427387968),
            },
            ByColor {
                black: Bitboard(7346272643315662848),
                white: Bitboard(338490213),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
