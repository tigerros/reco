use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
use reco_core::{Code, Opening, Volume};
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

/// King's Gambit Declined: Classical Variation, Euwe Attack.
pub const EUWE_ATTACK: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<30>(),
    },
    name: "King's Gambit Declined",
    variation: &["Classical Variation", "Euwe Attack"],
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
            role: Bishop,
            from: F8,
            capture: None,
            to: C5,
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
            from: D7,
            capture: None,
            to: D6,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: C2,
            capture: None,
            to: C3,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C8,
            capture: None,
            to: G4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: F4,
            capture: Some(Pawn),
            to: E5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D6,
            capture: Some(Pawn),
            to: E5,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: None,
            to: A4,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65020788608387840),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(18253611044),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752320200704),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(15845633900899467264),
                white: Bitboard(287624119),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
