use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
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

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Sicilian Defense.
pub static SICILIAN_DEFENSE: [Opening<&str>; 4] = [
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<20>(),
        },
        name: Cow::Borrowed(&["Sicilian Defense"]),
        moves: Cow::Borrowed(&[
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
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(70650236602740480),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18445336716005867520),
                    white: Bitboard(268496895),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<27>(),
        },
        name: Cow::Borrowed(&["Sicilian Defense"]),
        moves: Cow::Borrowed(&[
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(70650236602740480),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18445336716005867520),
                    white: Bitboard(270593983),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<50>(),
        },
        name: Cow::Borrowed(&["Sicilian Defense"]),
        moves: Cow::Borrowed(&[
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
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
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68407215836423936),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443093695239553024),
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
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<50>(),
        },
        name: Cow::Borrowed(&["Sicilian Defense"]),
        moves: Cow::Borrowed(&[
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
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
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68407233016293120),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443093712285204480),
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
        }),
    },
];
pub mod accelerated_dragon;
pub use accelerated_dragon::ACCELERATED_DRAGON;
pub mod acton_extension;
pub use acton_extension::ACTON_EXTENSION;
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod amazon_attack;
pub use amazon_attack::AMAZON_ATTACK;
pub mod big_clamp_formation;
pub use big_clamp_formation::BIG_CLAMP_FORMATION;
pub mod boleslavsky_variation;
pub use boleslavsky_variation::BOLESLAVSKY_VARIATION;
pub mod bowdler_attack;
pub use bowdler_attack::BOWDLER_ATTACK;
pub mod brick_variation;
pub use brick_variation::BRICK_VARIATION;
pub mod brussels_gambit;
pub use brussels_gambit::BRUSSELS_GAMBIT;
pub mod bucker_variation;
pub use bucker_variation::BUCKER_VARIATION;
pub mod chekhover_variation;
pub use chekhover_variation::CHEKHOVER_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod closed;
pub use closed::CLOSED;
pub mod coles_sicilian_gambit;
pub use coles_sicilian_gambit::COLES_SICILIAN_GAMBIT;
pub mod delayed_alapin_variation;
pub use delayed_alapin_variation::DELAYED_ALAPIN_VARIATION;
pub mod double_dutch_gambit;
pub use double_dutch_gambit::DOUBLE_DUTCH_GAMBIT;
pub mod dragon_variation;
pub use dragon_variation::DRAGON_VARIATION;
pub mod drazic_variation;
pub use drazic_variation::DRAZIC_VARIATION;
pub mod euwe_attack;
pub mod flohr_variation;
pub use flohr_variation::FLOHR_VARIATION;
pub mod four_knights_variation;
pub use four_knights_variation::FOUR_KNIGHTS_VARIATION;
pub mod franco_sicilian_variation;
pub use franco_sicilian_variation::FRANCO_SICILIAN_VARIATION;
pub mod frederico_variation;
pub use frederico_variation::FREDERICO_VARIATION;
pub mod french_variation;
pub use french_variation::FRENCH_VARIATION;
pub mod gaw_paw_variation;
pub use gaw_paw_variation::GAW_PAW_VARIATION;
pub mod gloria_variation;
pub use gloria_variation::GLORIA_VARIATION;
pub mod godiva_variation;
pub use godiva_variation::GODIVA_VARIATION;
pub mod grand_prix_attack;
pub use grand_prix_attack::GRAND_PRIX_ATTACK;
pub mod grob_variation;
pub use grob_variation::GROB_VARIATION;
pub mod halasz_gambit;
pub use halasz_gambit::HALASZ_GAMBIT;
pub mod heidenfeld_variation;
pub use heidenfeld_variation::HEIDENFELD_VARIATION;
pub mod hyperaccelerated_dragon;
pub use hyperaccelerated_dragon::HYPERACCELERATED_DRAGON;
pub mod hyperaccelerated_fianchetto;
pub use hyperaccelerated_fianchetto::HYPERACCELERATED_FIANCHETTO;
pub mod hyperaccelerated_pterodactyl;
pub use hyperaccelerated_pterodactyl::HYPERACCELERATED_PTERODACTYL;
pub mod jalalabad_variation;
pub use jalalabad_variation::JALALABAD_VARIATION;
pub mod kalashnikov_variation;
pub use kalashnikov_variation::KALASHNIKOV_VARIATION;
pub mod kan_variation;
pub use kan_variation::KAN_VARIATION;
pub mod katalimov_variation;
pub use katalimov_variation::KATALIMOV_VARIATION;
pub mod keres_variation;
pub use keres_variation::KERES_VARIATION;
pub mod king_davids_opening;
pub use king_davids_opening::KING_DAVIDS_OPENING;
pub mod kopec_system;
pub use kopec_system::KOPEC_SYSTEM;
pub mod kotov_gambit;
pub use kotov_gambit::KOTOV_GAMBIT;
pub mod kramnik_variation;
pub use kramnik_variation::KRAMNIK_VARIATION;
pub mod kronberger_variation;
pub use kronberger_variation::KRONBERGER_VARIATION;
pub mod kupreichik_variation;
pub use kupreichik_variation::KUPREICHIK_VARIATION;
pub mod kveinis_variation;
pub use kveinis_variation::KVEINIS_VARIATION;
pub mod lasker_dunne_attack;
pub use lasker_dunne_attack::LASKER_DUNNE_ATTACK;
pub mod lasker_pelikan_variation;
pub use lasker_pelikan_variation::LASKER_PELIKAN_VARIATION;
pub mod lowenthal_variation;
pub use lowenthal_variation::LOWENTHAL_VARIATION;
pub mod magnus_smith_trap;
pub use magnus_smith_trap::MAGNUS_SMITH_TRAP;
pub mod marshall_counterattack;
pub use marshall_counterattack::MARSHALL_COUNTERATTACK;
pub mod mc_donnell_attack;
pub use mc_donnell_attack::MC_DONNELL_ATTACK;
pub mod mengarini_variation;
pub use mengarini_variation::MENGARINI_VARIATION;
pub mod modern_variations;
pub use modern_variations::MODERN_VARIATIONS;
pub mod mongoose_variation;
pub use mongoose_variation::MONGOOSE_VARIATION;
pub mod morphy_gambit;
pub use morphy_gambit::MORPHY_GAMBIT;
pub mod moscow_variation;
pub use moscow_variation::MOSCOW_VARIATION;
pub mod myers_attack;
pub mod najdorf_variation;
pub use najdorf_variation::NAJDORF_VARIATION;
pub mod nimzo_american_variation;
pub use nimzo_american_variation::NIMZO_AMERICAN_VARIATION;
pub mod nimzowitsch_variation;
pub use nimzowitsch_variation::NIMZOWITSCH_VARIATION;
pub mod nyezhmetdinov_rossolimo_attack;
pub use nyezhmetdinov_rossolimo_attack::NYEZHMETDINOV_ROSSOLIMO_ATTACK;
pub mod o_kelly_variation;
pub use o_kelly_variation::O_KELLY_VARIATION;
pub mod old_sicilian;
pub use old_sicilian::OLD_SICILIAN;
pub mod open;
pub use open::OPEN;
pub mod paulsen_basman_defense;
pub use paulsen_basman_defense::PAULSEN_BASMAN_DEFENSE;
pub mod pin_variation;
pub use pin_variation::PIN_VARIATION;
pub mod polish_gambit;
pub use polish_gambit::POLISH_GAMBIT;
pub mod portsmouth_gambit;
pub use portsmouth_gambit::PORTSMOUTH_GAMBIT;
pub mod prins_variation;
pub use prins_variation::PRINS_VARIATION;
pub mod quinteros_variation;
pub use quinteros_variation::QUINTEROS_VARIATION;
pub mod richter_rauzer_variation;
pub use richter_rauzer_variation::RICHTER_RAUZER_VARIATION;
pub mod scheveningen_variation;
pub use scheveningen_variation::SCHEVENINGEN_VARIATION;
pub mod smith_morra_gambit;
pub use smith_morra_gambit::SMITH_MORRA_GAMBIT;
pub mod smith_morra_gambit_accepted;
pub use smith_morra_gambit_accepted::SMITH_MORRA_GAMBIT_ACCEPTED;
pub mod smith_morra_gambit_declined;
pub mod smith_morra_gambit_deferred;
pub use smith_morra_gambit_deferred::SMITH_MORRA_GAMBIT_DEFERRED;
pub mod snyder_variation;
pub use snyder_variation::SNYDER_VARIATION;
pub mod sozin_attack;
pub use sozin_attack::SOZIN_ATTACK;
pub mod spielmann_variation;
pub use spielmann_variation::SPIELMANN_VARIATION;
pub mod staunton_cochrane_variation;
pub use staunton_cochrane_variation::STAUNTON_COCHRANE_VARIATION;
pub mod taimanov_variation;
pub use taimanov_variation::TAIMANOV_VARIATION;
pub mod velimirovic_attack;
pub use velimirovic_attack::VELIMIROVIC_ATTACK;
pub mod venice_attack;
pub use venice_attack::VENICE_ATTACK;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod wing_gambit_deferred;
pub use wing_gambit_deferred::WING_GAMBIT_DEFERRED;
pub mod yates_variation;
pub use yates_variation::YATES_VARIATION;
