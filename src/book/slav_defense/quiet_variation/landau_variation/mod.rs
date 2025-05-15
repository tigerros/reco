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
/// Slav Defense: Quiet Variation, Landau Variation.
pub const LANDAU_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<12>(),
    },
    name: "Slav Defense",
    variation: &["Quiet Variation", "Landau Variation"],
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
            from: C7,
            capture: None,
            to: C6,
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
            from: G8,
            capture: None,
            to: F6,
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
            from: C8,
            capture: None,
            to: F5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C4,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C6,
            capture: Some(Pawn),
            to: D5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: B3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: C8,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C1,
            capture: None,
            to: D2,
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
            to: A3,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912446394426112),
                knight: Bitboard(144150372450107392),
                bishop: Bitboard(2305843146652649504),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(288230376151842816),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13250487477010956288),
                white: Bitboard(137620401),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
