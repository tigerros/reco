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
/// Pterodactyl Defense: Fianchetto, Queen Pteranodon.
pub const QUEEN_PTERANODON: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<40>(),
    },
    name: "Pterodactyl Defense",
    variation: &["Fianchetto", "Queen Pteranodon"],
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
            from: G7,
            capture: None,
            to: G6,
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
            role: Bishop,
            from: F8,
            capture: None,
            to: G7,
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
            role: Pawn,
            from: C7,
            capture: None,
            to: C5,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D4,
            capture: None,
            to: D5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: G7,
            capture: Some(Knight),
            to: C3,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: B2,
            capture: Some(Bishop),
            to: C3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: A5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(52706240996110592),
                knight: Bitboard(4755801206503243840),
                bishop: Bitboard(288230376151711780),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(4294967304),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(15545088929018413056),
                white: Bitboard(34427171325),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
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
