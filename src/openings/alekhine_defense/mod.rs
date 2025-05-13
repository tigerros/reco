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

/// Alekhine Defense.
pub const ALEKHINE_DEFENSE: [Opening<'static, &str>; 4] = [
    Opening {
        code: Code {
            volume: Volume::B,
            subcategory: RangedU8::new_static::<2>(),
        },
        name: "Alekhine Defense",
        variation: &[],
        moves: &[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                    pawn: Bitboard(71776119329713920),
                    knight: Bitboard(144150372447944770),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13834811764677541888),
                    white: Bitboard(268496895),
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
            volume: Volume::B,
            subcategory: RangedU8::new_static::<3>(),
        },
        name: "Alekhine Defense",
        variation: &[],
        moves: &[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                from: E4,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776187914970880),
                    knight: Bitboard(144115222435594306),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13834776614665191424),
                    white: Bitboard(68853753855),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
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
            volume: Volume::B,
            subcategory: RangedU8::new_static::<3>(),
        },
        name: "Alekhine Defense",
        variation: &[],
        moves: &[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                from: E4,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: None,
                to: D5,
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
                    pawn: Bitboard(69533184194307840),
                    knight: Bitboard(144115222435594306),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13832533610944528384),
                    white: Bitboard(68853753855),
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
    },
    Opening {
        code: Code {
            volume: Volume::B,
            subcategory: RangedU8::new_static::<3>(),
        },
        name: "Alekhine Defense",
        variation: &[],
        moves: &[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                from: E4,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: None,
                to: D5,
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
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69533184261415680),
                    knight: Bitboard(144115222435594306),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13832533610944528384),
                    white: Bitboard(68920861695),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
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
    },
];
pub mod brooklyn_variation;
pub use brooklyn_variation::BROOKLYN_VARIATION;
pub mod steiner_variation;
pub use steiner_variation::STEINER_VARIATION;
pub mod welling_variation;
pub use welling_variation::WELLING_VARIATION;
pub mod krejcik_variation;
pub use krejcik_variation::KREJCIK_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod scandinavian_variation;
pub use scandinavian_variation::SCANDINAVIAN_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod two_pawns_attack;
pub use two_pawns_attack::TWO_PAWNS_ATTACK;
pub mod o_sullivan_gambit;
pub use o_sullivan_gambit::O_SULLIVAN_GAMBIT;
pub mod hunt_variation;
pub use hunt_variation::HUNT_VARIATION;
pub mod balogh_variation;
pub use balogh_variation::BALOGH_VARIATION;
pub mod spielmann_gambit;
pub use spielmann_gambit::SPIELMANN_GAMBIT;
pub mod samisch_attack;
pub use samisch_attack::SAMISCH_ATTACK;
pub mod kmoch_variation;
pub use kmoch_variation::KMOCH_VARIATION;
pub mod mokele_mbembe;
pub use mokele_mbembe::MOKELE_MBEMBE;
pub mod the_squirrel;
pub use the_squirrel::THE_SQUIRREL;
pub mod buckley_attack;
pub use buckley_attack::BUCKLEY_ATTACK;
pub mod four_pawns_attack;
pub use four_pawns_attack::FOUR_PAWNS_ATTACK;
pub mod maroczy_variation;
pub use maroczy_variation::MAROCZY_VARIATION;
