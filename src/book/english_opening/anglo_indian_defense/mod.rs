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

/// English Opening: Anglo-Indian Defense.
pub const ANGLO_INDIAN_DEFENSE: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<15>(),
        },
        name: "English Opening",
        variation: &["Anglo-Indian Defense"],
        moves: &[
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C4,
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
                    pawn: Bitboard(71776119128390400),
                    knight: Bitboard(144150372447944770),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13834811764677541888),
                    white: Bitboard(67173375),
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
            volume: Volume::A,
            category: RangedU8::new_static::<17>(),
        },
        name: "English Opening",
        variation: &["Anglo-Indian Defense"],
        moves: &[
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C4,
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
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C4,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: Some(Pawn),
                to: D5,
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
                from: C7,
                capture: None,
                to: C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E3,
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
                    pawn: Bitboard(63912429080341248),
                    knight: Bitboard(144115222437953536),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13826912924683206656),
                    white: Bitboard(3468221),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
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
];
pub mod anglo_grunfeld_variation;
pub use anglo_grunfeld_variation::ANGLO_GRUNFELD_VARIATION;
pub mod anti_anti_grunfeld;
pub use anti_anti_grunfeld::ANTI_ANTI_GRUNFELD;
pub mod flohr_mikenas_carls_variation;
pub mod grunfeld_formation;
pub use grunfeld_formation::GRUNFELD_FORMATION;
pub mod hedgehog_system;
pub use hedgehog_system::HEDGEHOG_SYSTEM;
pub mod kings_indian_formation;
pub use kings_indian_formation::KINGS_INDIAN_FORMATION;
pub mod kings_knight_variation;
pub use kings_knight_variation::KINGS_KNIGHT_VARIATION;
pub mod nimzo_english;
pub use nimzo_english::NIMZO_ENGLISH;
pub mod old_indian_formation;
pub use old_indian_formation::OLD_INDIAN_FORMATION;
pub mod queens_indian_formation;
pub use queens_indian_formation::QUEENS_INDIAN_FORMATION;
pub mod queens_indian_variation;
pub use queens_indian_variation::QUEENS_INDIAN_VARIATION;
pub mod queens_knight_variation;
pub use queens_knight_variation::QUEENS_KNIGHT_VARIATION;
pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;
pub mod scandinavian_defense;
pub use scandinavian_defense::SCANDINAVIAN_DEFENSE;
pub mod slav_formation;
pub use slav_formation::SLAV_FORMATION;
pub mod zvjaginsev_krasenkow_attack;
pub use zvjaginsev_krasenkow_attack::ZVJAGINSEV_KRASENKOW_ATTACK;
