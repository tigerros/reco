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

/// Queen's Gambit Declined.
pub const QUEENS_GAMBIT_DECLINED: [Opening<'static, &str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<30>(),
        },
        name: "Queen's Gambit Declined",
        variation: &[],
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
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346367333120),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18439724825837568000),
                    white: Bitboard(201389055),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<52>(),
        },
        name: "Queen's Gambit Declined",
        variation: &[],
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
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
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: D7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63916844508046080),
                    knight: Bitboard(2286984188133376),
                    bishop: Bitboard(2594073660243312672),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13685089101659766784),
                    white: Bitboard(275082699705),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<53>(),
        },
        name: "Queen's Gambit Declined",
        variation: &[],
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
                to: E7,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346367333120),
                    knight: Bitboard(144150372448206912),
                    bishop: Bitboard(292734250656989216),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(11526734582195945472),
                    white: Bitboard(275079558137),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
pub mod austrian_attack;
pub mod hastings_variation;
pub use hastings_variation::HASTINGS_VARIATION;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod semi_tarrasch_defense;
pub use semi_tarrasch_defense::SEMI_TARRASCH_DEFENSE;
pub mod barmen_variation;
pub use barmen_variation::BARMEN_VARIATION;
pub mod austrian_defense;
pub use austrian_defense::AUSTRIAN_DEFENSE;
pub mod westphalian_variation;
pub use westphalian_variation::WESTPHALIAN_VARIATION;
pub mod ragozin_defense;
pub use ragozin_defense::RAGOZIN_DEFENSE;
pub mod stonewall_variation;
pub use stonewall_variation::STONEWALL_VARIATION;
pub mod miles_variation;
pub use miles_variation::MILES_VARIATION;
pub mod charousek_variation;
pub use charousek_variation::CHAROUSEK_VARIATION;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod rochlin_variation;
pub use rochlin_variation::ROCHLIN_VARIATION;
pub mod semmering_variation;
pub use semmering_variation::SEMMERING_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod marshall_defense;
pub use marshall_defense::MARSHALL_DEFENSE;
pub mod exchange_variation;
pub mod semi_slav;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod three_knights;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod lasker_defense;
pub use lasker_defense::LASKER_DEFENSE;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod neo_orthodox_variation;
pub use neo_orthodox_variation::NEO_ORTHODOX_VARIATION;
pub mod pseudo_tarrasch_variation;
pub use pseudo_tarrasch_variation::PSEUDO_TARRASCH_VARIATION;
pub mod normal_defense;
pub use normal_defense::NORMAL_DEFENSE;
pub mod orthodox_defense;
pub use orthodox_defense::ORTHODOX_DEFENSE;
pub mod harrwitz_attack;
pub use harrwitz_attack::HARRWITZ_ATTACK;
pub mod uhlmann_variation;
pub use uhlmann_variation::UHLMANN_VARIATION;
pub mod tartakower_defense;
pub use tartakower_defense::TARTAKOWER_DEFENSE;
pub mod cambridge_springs_defense;
pub mod knight_defense;
pub use cambridge_springs_defense::CAMBRIDGE_SPRINGS_DEFENSE;
pub mod tarrasch_defense;
pub use tarrasch_defense::TARRASCH_DEFENSE;
pub mod manhattan_variation;
pub use manhattan_variation::MANHATTAN_VARIATION;
pub mod queens_knight_variation;
pub use queens_knight_variation::QUEENS_KNIGHT_VARIATION;
pub mod spielmann_variation;
pub use spielmann_variation::SPIELMANN_VARIATION;
pub mod pillsbury_attack;
pub use pillsbury_attack::PILLSBURY_ATTACK;
pub mod been_koomen_variation;
pub use been_koomen_variation::BEEN_KOOMEN_VARIATION;
pub mod anti_tartakower_variation;
pub use anti_tartakower_variation::ANTI_TARTAKOWER_VARIATION;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
pub mod vienna_variation;
pub use vienna_variation::VIENNA_VARIATION;
pub mod albin_countergambit;
pub use albin_countergambit::ALBIN_COUNTERGAMBIT;
pub mod janowski_variation;
pub use janowski_variation::JANOWSKI_VARIATION;
pub mod baltic_defense;
pub use baltic_defense::BALTIC_DEFENSE;
