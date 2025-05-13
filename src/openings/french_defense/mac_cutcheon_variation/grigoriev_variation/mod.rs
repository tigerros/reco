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

/// French Defense: MacCutcheon Variation, Grigoriev Variation.
pub const GRIGORIEV_VARIATION: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        subcategory: RangedU8::new_static::<12>(),
    },
    name: "French Defense",
    variation: &["MacCutcheon Variation", "Grigoriev Variation"],
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
            role: Bishop,
            from: C1,
            capture: None,
            to: G5,
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
            role: Pawn,
            from: E4,
            capture: None,
            to: E5,
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
            role: Pawn,
            from: E5,
            capture: Some(Knight),
            to: F6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: H6,
            capture: Some(Bishop),
            to: G5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: F6,
            capture: Some(Pawn),
            to: G7,
            promotion: None,
        },
        Normal {
            role: Rook,
            from: H8,
            capture: None,
            to: G8,
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
            capture: Some(Pawn),
            to: H4,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: G4,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(29009551428708096),
                knight: Bitboard(144115188076118080),
                bishop: Bitboard(288230376185266208),
                rook: Bitboard(4683743612465315969),
                queen: Bitboard(576460753377165312),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(6856466586421690368),
                white: Bitboard(18014399717730289),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(72057594037928065),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
