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
/// Indian Defense.
pub const INDIAN_DEFENSE: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<45>(),
        },
        name: &["Indian Defense"],
        moves: &[
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119195498240),
                    knight: Bitboard(144150372447944770),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13834811764677541888),
                    white: Bitboard(134281215),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::E,
            category: RangedU8::new_static::<0>(),
        },
        name: &["Indian Defense"],
        moves: &[
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
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
                role: Queen,
                from: D1,
                capture: None,
                to: B3,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67290111821280000),
                    knight: Bitboard(144150372447944770),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303554560),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13830325757236215808),
                    white: Bitboard(201520119),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
pub mod anti_grunfeld;
pub mod anti_nimzo_indian;
pub use anti_nimzo_indian::ANTI_NIMZO_INDIAN;
pub mod budapest_defense;
pub use budapest_defense::BUDAPEST_DEFENSE;
pub mod colle_system;
pub mod czech_indian;
pub use czech_indian::CZECH_INDIAN;
pub mod devin_gambit;
pub use devin_gambit::DEVIN_GAMBIT;
pub mod dory_indian;
pub use dory_indian::DORY_INDIAN;
pub mod dzindzi_indian_defense;
pub use dzindzi_indian_defense::DZINDZI_INDIAN_DEFENSE;
pub mod gedult_attack;
pub use gedult_attack::GEDULT_ATTACK;
pub mod gibbins_weidenhagen_gambit;
pub use gibbins_weidenhagen_gambit::GIBBINS_WEIDENHAGEN_GAMBIT;
pub mod gibbins_weidenhagen_gambit_accepted;
pub use gibbins_weidenhagen_gambit_accepted::GIBBINS_WEIDENHAGEN_GAMBIT_ACCEPTED;
pub mod kings_indian_variation;
pub mod knights_variation;
pub use knights_variation::KNIGHTS_VARIATION;
pub mod lazard_gambit;
pub use lazard_gambit::LAZARD_GAMBIT;
pub mod london_system;
pub use london_system::LONDON_SYSTEM;
pub mod maddigan_gambit;
pub use maddigan_gambit::MADDIGAN_GAMBIT;
pub mod medusa_gambit;
pub use medusa_gambit::MEDUSA_GAMBIT;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod omega_gambit;
pub use omega_gambit::OMEGA_GAMBIT;
pub mod paleface_attack;
pub mod pawn_push_variation;
pub use pawn_push_variation::PAWN_PUSH_VARIATION;
pub mod polish_variation;
pub use polish_variation::POLISH_VARIATION;
pub mod przepiorka_variation;
pub use przepiorka_variation::PRZEPIORKA_VARIATION;
pub mod pseudo_benko;
pub use pseudo_benko::PSEUDO_BENKO;
pub mod pyrenees_gambit;
pub use pyrenees_gambit::PYRENEES_GAMBIT;
pub mod reversed_chigorin_defense;
pub use reversed_chigorin_defense::REVERSED_CHIGORIN_DEFENSE;
pub mod schnepper_gambit;
pub use schnepper_gambit::SCHNEPPER_GAMBIT;
pub mod seirawan_attack;
pub use seirawan_attack::SEIRAWAN_ATTACK;
pub mod spielmann_indian;
pub use spielmann_indian::SPIELMANN_INDIAN;
pub mod tartakower_attack;
pub use tartakower_attack::TARTAKOWER_ATTACK;
pub mod wade_tartakower_defense;
pub use wade_tartakower_defense::WADE_TARTAKOWER_DEFENSE;
pub mod west_indian_defense;
pub use west_indian_defense::WEST_INDIAN_DEFENSE;
