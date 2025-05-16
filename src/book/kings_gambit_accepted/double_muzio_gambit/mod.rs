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
/// King's Gambit Accepted: Double Muzio Gambit.
pub const DOUBLE_MUZIO_GAMBIT: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<37>(),
    },
    name: &["King's Gambit Accepted", "Double Muzio Gambit"],
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
        Castle { king: E1, rook: H1 },
        Normal {
            role: Pawn,
            from: G4,
            capture: Some(Knight),
            to: F3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D1,
            capture: Some(Pawn),
            to: F3,
            promotion: None,
        },
        Normal {
            role: Queen,
            from: D8,
            capture: None,
            to: F6,
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
            role: Queen,
            from: F6,
            capture: Some(Pawn),
            to: E5,
            promotion: None,
        },
        Normal {
            role: Bishop,
            from: C4,
            capture: Some(Pawn),
            to: F7,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(40250922206547712),
                knight: Bitboard(4755801206503243778),
                bishop: Bitboard(2603080584620146692),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(68721573888),
                king: Bitboard(1152921504606847040),
            },
            ByColor {
                black: Bitboard(17838476718294171648),
                white: Bitboard(9007199256891239),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
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
pub mod baldwin_gambit;
pub use baldwin_gambit::BALDWIN_GAMBIT;
pub mod bello_gambit;
pub use bello_gambit::BELLO_GAMBIT;
pub mod paulsen_defense;
pub use paulsen_defense::PAULSEN_DEFENSE;
pub mod young_gambit;
pub use young_gambit::YOUNG_GAMBIT;
