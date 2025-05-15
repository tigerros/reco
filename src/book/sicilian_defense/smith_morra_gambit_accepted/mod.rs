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

/// Sicilian Defense: Smith-Morra Gambit Accepted.
pub const SMITH_MORRA_GAMBIT_ACCEPTED: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::B,
        category: RangedU8::new_static::<21>(),
    },
    name: "Sicilian Defense",
    variation: &["Smith-Morra Gambit Accepted"],
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
            from: C7,
            capture: None,
            to: C5,
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
            from: C5,
            capture: Some(Pawn),
            to: D4,
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
            role: Pawn,
            from: D4,
            capture: Some(Pawn),
            to: C3,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(70650219423130368),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18445336698826260480),
                white: Bitboard(268493823),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
pub mod chicago_defense;
pub use chicago_defense::CHICAGO_DEFENSE;
pub mod classical_formation;
pub use classical_formation::CLASSICAL_FORMATION;
pub mod danish_variation;
pub use danish_variation::DANISH_VARIATION;
pub mod fianchetto_defense;
pub use fianchetto_defense::FIANCHETTO_DEFENSE;
pub mod finegold_defense;
pub use finegold_defense::FINEGOLD_DEFENSE;
pub mod kan_formation;
pub use kan_formation::KAN_FORMATION;
pub mod larsen_defense;
pub use larsen_defense::LARSEN_DEFENSE;
pub mod morphy_defense;
pub use morphy_defense::MORPHY_DEFENSE;
pub mod morphy_defense_deferred;
pub use morphy_defense_deferred::MORPHY_DEFENSE_DEFERRED;
pub mod paulsen_formation;
pub use paulsen_formation::PAULSEN_FORMATION;
pub mod pin_defense;
pub use pin_defense::PIN_DEFENSE;
pub mod scheveningen_formation;
pub use scheveningen_formation::SCHEVENINGEN_FORMATION;
pub mod siberian_variation;
pub use siberian_variation::SIBERIAN_VARIATION;
pub mod sozin_formation;
pub use sozin_formation::SOZIN_FORMATION;
pub mod taimanov_formation;
pub use taimanov_formation::TAIMANOV_FORMATION;
