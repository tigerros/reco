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
/// King's Gambit Accepted: Salvio Gambit, Anderssen Counterattack.
pub const ANDERSSEN_COUNTERATTACK: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<37>(),
    },
    name: "King's Gambit Accepted",
    variation: &["Salvio Gambit", "Anderssen Counterattack"],
    moves: &[
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
            capture: Some(Pawn),
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
            role: Bishop,
            from: F1,
            capture: None,
            to: C4,
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
            to: E5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: H4,
            promotion: None,
        },
        Normal {
            role: King,
            from: E1,
            capture: None,
            to: F1,
            promotion: None,
        },
        Normal {
            role: Knight,
            from: G8,
            capture: None,
            to: H6,
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
            to: D6,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(47015119217018624),
                knight: Bitboard(144255994283687938),
                bishop: Bitboard(2594073385432514564),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(2147483656),
                king: Bitboard(1152921504606847008),
            },
            ByColor {
                black: Bitboard(13233695567390965760),
                white: Bitboard(69189289903),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
