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
/// Blackmar-Diemer Gambit Accepted.
pub const BLACKMAR_DIEMER_GAMBIT_ACCEPTED: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<0>(),
    },
    name: &["Blackmar-Diemer Gambit Accepted"],
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
            from: E2,
            capture: None,
            to: E4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D5,
            capture: Some(Pawn),
            to: E4,
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
            role: Pawn,
            from: F2,
            capture: None,
            to: F3,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E4,
            capture: Some(Pawn),
            to: F3,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(69524319383897856),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13832559964865953792),
                white: Bitboard(134531069),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
pub mod bogoljubow_defense;
pub use bogoljubow_defense::BOGOLJUBOW_DEFENSE;
pub mod euwe_defense;
pub use euwe_defense::EUWE_DEFENSE;
pub mod gunderam_defense;
pub use gunderam_defense::GUNDERAM_DEFENSE;
pub mod holwell_defense;
pub use holwell_defense::HOLWELL_DEFENSE;
pub mod kaulich_defense;
pub use kaulich_defense::KAULICH_DEFENSE;
pub mod pietrowsky_defense;
pub use pietrowsky_defense::PIETROWSKY_DEFENSE;
pub mod ritter_defense;
pub use ritter_defense::RITTER_DEFENSE;
pub mod ryder_gambit;
pub use ryder_gambit::RYDER_GAMBIT;
pub mod schlutter_defense;
pub use schlutter_defense::SCHLUTTER_DEFENSE;
pub mod teichmann_defense;
pub use teichmann_defense::TEICHMANN_DEFENSE;
pub mod ziegler_defense;
pub use ziegler_defense::ZIEGLER_DEFENSE;
