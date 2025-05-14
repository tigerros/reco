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

/// French Defense.
pub const FRENCH_DEFENSE: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<0>(),
        },
        name: "French Defense",
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
                to: E6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67290111888387840),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441976591291514880),
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
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<0>(),
        },
        name: "French Defense",
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
                to: E6,
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
                to: D5,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346568656640),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18439724825837568000),
                    white: Bitboard(402712575),
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
];
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod banzai_leong_gambit;
pub use banzai_leong_gambit::BANZAI_LEONG_GAMBIT;
pub mod alekhine_chatard_attack;
pub use alekhine_chatard_attack::ALEKHINE_CHATARD_ATTACK;
pub mod morphy_gambit;
pub use morphy_gambit::MORPHY_GAMBIT;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod reversed_philidor_formation;
pub use reversed_philidor_formation::REVERSED_PHILIDOR_FORMATION;
pub mod baeuerle_gambit;
pub use baeuerle_gambit::BAEUERLE_GAMBIT;
pub mod guimard_variation;
pub mod mac_cutcheon_variation;
pub use mac_cutcheon_variation::MAC_CUTCHEON_VARIATION;
pub mod alapin_gambit;
pub use alapin_gambit::ALAPIN_GAMBIT;
pub mod chigorin_variation;
pub use chigorin_variation::CHIGORIN_VARIATION;
pub mod horwitz_attack;
pub use horwitz_attack::HORWITZ_ATTACK;
pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
pub mod steiner_variation;
pub use steiner_variation::STEINER_VARIATION;
pub mod franco_hiva_gambit_accepted;
pub use franco_hiva_gambit_accepted::FRANCO_HIVA_GAMBIT_ACCEPTED;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod reti_spielmann_attack;
pub use reti_spielmann_attack::RETI_SPIELMANN_ATTACK;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod franco_hiva_gambit;
pub use franco_hiva_gambit::FRANCO_HIVA_GAMBIT;
pub mod steinitz_attack;
pub use steinitz_attack::STEINITZ_ATTACK;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod mediterranean_defense;
pub use mediterranean_defense::MEDITERRANEAN_DEFENSE;
pub mod orthoschnapp_gambit;
pub use orthoschnapp_gambit::ORTHOSCHNAPP_GAMBIT;
pub mod diemer_duhm_gambit_accepted;
pub use diemer_duhm_gambit_accepted::DIEMER_DUHM_GAMBIT_ACCEPTED;
pub mod kings_indian_attack;
pub use kings_indian_attack::KINGS_INDIAN_ATTACK;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod knight_variation;
pub use knight_variation::KNIGHT_VARIATION;
pub mod winawer_variation;
pub use winawer_variation::WINAWER_VARIATION;
pub mod perseus_gambit;
pub use perseus_gambit::PERSEUS_GAMBIT;
pub mod carlson_gambit;
pub use carlson_gambit::CARLSON_GAMBIT;
pub mod hoffmann_gambit;
pub use hoffmann_gambit::HOFFMANN_GAMBIT;
pub mod franco_sicilian_defense;
pub use franco_sicilian_defense::FRANCO_SICILIAN_DEFENSE;
pub mod burn_variation;
pub use burn_variation::BURN_VARIATION;
pub mod henneberger_variation;
pub use henneberger_variation::HENNEBERGER_VARIATION;
pub mod queens_knight;
pub use queens_knight::QUEENS_KNIGHT;
pub mod st_george_defense;
pub use st_george_defense::ST_GEORGE_DEFENSE;
pub mod tarrasch_variation;
pub use tarrasch_variation::TARRASCH_VARIATION;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod bird_invitation;
pub use bird_invitation::BIRD_INVITATION;
pub mod diemer_duhm_gambit;
pub use diemer_duhm_gambit::DIEMER_DUHM_GAMBIT;
pub mod la_bourdonnais_variation;
pub use la_bourdonnais_variation::LA_BOURDONNAIS_VARIATION;
pub mod schlechter_variation;
pub use schlechter_variation::SCHLECHTER_VARIATION;
pub mod pelikan_variation;
pub use pelikan_variation::PELIKAN_VARIATION;
