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

/// Italian Game: Evans Gambit.
pub const EVANS_GAMBIT: [Opening<'static, &str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<51>(),
        },
        name: "Italian Game",
        variation: &["Evans Gambit"],
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
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
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
                role: Pawn,
                from: B2,
                capture: None,
                to: B4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588455374080),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(288230393398689796),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(15992005285761777664),
                    white: Bitboard(371256735),
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
            category: RangedU8::new_static::<51>(),
        },
        name: "Italian Game",
        variation: &["Evans Gambit"],
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
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
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
                role: Pawn,
                from: B2,
                capture: None,
                to: B4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: Some(Pawn),
                to: B4,
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
                from: B4,
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
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C3,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: None,
                to: B6,
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
                role: Bishop,
                from: C8,
                capture: None,
                to: G4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65029516115894528),
                    knight: Bitboard(4611690416476258304),
                    bishop: Bitboard(2200164106244),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(15701534020087054336),
                    white: Bitboard(472179053),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 3,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<52>(),
        },
        name: "Italian Game",
        variation: &["Evans Gambit"],
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
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
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
                role: Pawn,
                from: B2,
                capture: None,
                to: B4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: Some(Pawn),
                to: B4,
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
                from: B4,
                capture: None,
                to: A5,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
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
                    pawn: Bitboard(65029584701417728),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(288230380513787908),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(15989762269156212736),
                    white: Bitboard(337963375),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
pub mod mc_donnell_defense;
pub use mc_donnell_defense::MC_DONNELL_DEFENSE;
pub mod leonhardt_countergambit;
pub use leonhardt_countergambit::LEONHARDT_COUNTERGAMBIT;
pub mod stone_ware_variation;
pub use stone_ware_variation::STONE_WARE_VARIATION;
pub mod sanders_alapin_variation;
pub use sanders_alapin_variation::SANDERS_ALAPIN_VARIATION;
pub mod levenfish_variation;
pub use levenfish_variation::LEVENFISH_VARIATION;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod sokolsky_variation;
pub use sokolsky_variation::SOKOLSKY_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod johner_defense;
pub use johner_defense::JOHNER_DEFENSE;
pub mod ulvestad_variation;
pub use ulvestad_variation::ULVESTAD_VARIATION;
pub mod fontaine_countergambit;
pub use fontaine_countergambit::FONTAINE_COUNTERGAMBIT;
pub mod alapin_steinitz_variation;
pub use alapin_steinitz_variation::ALAPIN_STEINITZ_VARIATION;
pub mod dufresne_defense;
pub use dufresne_defense::DUFRESNE_DEFENSE;
pub mod laroche_variation;
pub use laroche_variation::LAROCHE_VARIATION;
pub mod mayet_defense;
pub use mayet_defense::MAYET_DEFENSE;
pub mod mortimer_evans_gambit;
pub use mortimer_evans_gambit::MORTIMER_EVANS_GAMBIT;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod mieses_defense;
pub use mieses_defense::MIESES_DEFENSE;
pub mod fraser_attack;
pub use fraser_attack::FRASER_ATTACK;
pub mod morphy_attack;
pub use morphy_attack::MORPHY_ATTACK;
pub mod tartakower_attack;
pub use tartakower_attack::TARTAKOWER_ATTACK;
pub mod goring_attack;
pub use goring_attack::GORING_ATTACK;
pub mod pierce_defense;
pub use pierce_defense::PIERCE_DEFENSE;
pub mod slow_variation;
pub use slow_variation::SLOW_VARIATION;
pub mod richardson_attack;
pub use richardson_attack::RICHARDSON_ATTACK;
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod hein_countergambit;
pub use hein_countergambit::HEIN_COUNTERGAMBIT;
pub mod harding_variation;
pub use harding_variation::HARDING_VARIATION;
pub mod lasker_defense;
pub use lasker_defense::LASKER_DEFENSE;
pub mod bronstein_defense;
pub use bronstein_defense::BRONSTEIN_DEFENSE;
pub mod waller_attack;
pub use waller_attack::WALLER_ATTACK;
pub mod fraser_mortimer_attack;
pub use fraser_mortimer_attack::FRASER_MORTIMER_ATTACK;
pub mod compromised_defense;
pub use compromised_defense::COMPROMISED_DEFENSE;
