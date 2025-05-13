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

/// Scotch Game.
pub const SCOTCH_GAME: [Opening<'static, &str>; 4] = [
    Opening {
        code: Code {
            volume: Volume::C,
            subcategory: RangedU8::new_static::<44>(),
        },
        name: "Scotch Game",
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
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
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
                from: B8,
                capture: None,
                to: C6,
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
                    pawn: Bitboard(67272588556035840),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18297848277795602432),
                    white: Bitboard(404809663),
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
            volume: Volume::C,
            subcategory: RangedU8::new_static::<44>(),
        },
        name: "Scotch Game",
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
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
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
                from: B8,
                capture: None,
                to: C6,
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
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272519836559104),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18297848209210343424),
                    white: Bitboard(270591935),
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
            volume: Volume::C,
            subcategory: RangedU8::new_static::<45>(),
        },
        name: "Scotch Game",
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
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
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
                from: B8,
                capture: None,
                to: C6,
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
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272519702341376),
                    knight: Bitboard(4611690416608116738),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18297848209076125696),
                    white: Bitboard(402712511),
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
    Opening {
        code: Code {
            volume: Volume::C,
            subcategory: RangedU8::new_static::<45>(),
        },
        name: "Scotch Game",
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
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
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
                from: B8,
                capture: None,
                to: C6,
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
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
                to: D4,
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
                role: Knight,
                from: D4,
                capture: None,
                to: B5,
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
                role: Bishop,
                from: C1,
                capture: None,
                to: D2,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272519702341376),
                    knight: Bitboard(4611690425063833602),
                    bishop: Bitboard(288230376185268256),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(2147483656),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(15415544449740046336),
                    white: Bitboard(8858431419),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];

pub mod lolli_variation;
pub use lolli_variation::LOLLI_VARIATION;

pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;

pub mod benima_defense;
pub use benima_defense::BENIMA_DEFENSE;

pub mod napoleon_gambit;
pub use napoleon_gambit::NAPOLEON_GAMBIT;

pub mod mieses_variation;
pub use mieses_variation::MIESES_VARIATION;

pub mod meitner_variation;
pub use meitner_variation::MEITNER_VARIATION;

pub mod scotch_gambit;
pub use scotch_gambit::SCOTCH_GAMBIT;

pub mod paulsen_attack;
pub use paulsen_attack::PAULSEN_ATTACK;

pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;

pub mod hanneken_variation;
pub use hanneken_variation::HANNEKEN_VARIATION;

pub mod vitzthum_attack;
pub use vitzthum_attack::VITZTHUM_ATTACK;

pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;

pub mod ghulam_kassim_variation;
pub use ghulam_kassim_variation::GHULAM_KASSIM_VARIATION;

pub mod rosenthal_variation;
pub use rosenthal_variation::ROSENTHAL_VARIATION;

pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;

pub mod cochrane_shumov_defense;
pub use cochrane_shumov_defense::COCHRANE_SHUMOV_DEFENSE;

pub mod gottschall_variation;
pub use gottschall_variation::GOTTSCHALL_VARIATION;

pub mod tartakower_variation;
pub use tartakower_variation::TARTAKOWER_VARIATION;

pub mod potter_variation;
pub use potter_variation::POTTER_VARIATION;

pub mod horwitz_attack;
pub use horwitz_attack::HORWITZ_ATTACK;

pub mod malaniuk_variation;
pub use malaniuk_variation::MALANIUK_VARIATION;

pub mod fraser_variation;
pub use fraser_variation::FRASER_VARIATION;

pub mod sea_cadet_mate;
pub use sea_cadet_mate::SEA_CADET_MATE;

pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;

pub mod blumenfeld_attack;
pub use blumenfeld_attack::BLUMENFELD_ATTACK;

pub mod haxo_gambit;
pub use haxo_gambit::HAXO_GAMBIT;

pub mod braune_variation;
pub use braune_variation::BRAUNE_VARIATION;

pub mod cochrane_variation;
pub use cochrane_variation::COCHRANE_VARIATION;

pub mod relfsson_gambit;
pub use relfsson_gambit::RELFSSON_GAMBIT;

pub mod goring_gambit;
pub use goring_gambit::GORING_GAMBIT;

pub mod schmidt_variation;
pub use schmidt_variation::SCHMIDT_VARIATION;

pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
