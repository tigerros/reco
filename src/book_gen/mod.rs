#![allow(
    clippy::allow_attributes,
    reason = "this module is generated, the allows don't know if they are going to be fulfilled"
)]
use crate::Variation;

#[cfg(test)]
mod tests {
    /// Asserts that each variation in [`super::ALL`] is valid.
    #[test]
    fn valid() {
        for variation in super::ALL {
            assert!(variation.validity().is_ok());
        }
    }
}
pub mod alekhine_defense;
pub use alekhine_defense::ALEKHINE_DEFENSE;
pub mod amar_opening;
pub use amar_opening::AMAR_OPENING;
pub mod amazon_attack;
pub use amazon_attack::AMAZON_ATTACK;
pub mod amsterdam_attack;
pub use amsterdam_attack::AMSTERDAM_ATTACK;
pub mod anderssens_opening;
pub use anderssens_opening::ANDERSSENS_OPENING;
pub mod australian_defense;
pub use australian_defense::AUSTRALIAN_DEFENSE;
pub mod barnes_defense;
pub use barnes_defense::BARNES_DEFENSE;
pub mod barnes_opening;
pub use barnes_opening::BARNES_OPENING;
pub mod benko_gambit;
pub use benko_gambit::BENKO_GAMBIT;
pub mod benko_gambit_accepted;
pub use benko_gambit_accepted::BENKO_GAMBIT_ACCEPTED;
pub mod benko_gambit_declined;
pub use benko_gambit_declined::BENKO_GAMBIT_DECLINED;
pub mod benoni_defense;
pub use benoni_defense::BENONI_DEFENSE;
pub mod bird_opening;
pub use bird_opening::BIRD_OPENING;
pub mod bishops_opening;
pub use bishops_opening::BISHOPS_OPENING;
pub mod blackmar_diemer_gambit;
pub use blackmar_diemer_gambit::BLACKMAR_DIEMER_GAMBIT;
pub mod blackmar_diemer_gambit_accepted;
pub use blackmar_diemer_gambit_accepted::BLACKMAR_DIEMER_GAMBIT_ACCEPTED;
pub mod blackmar_diemer_gambit_declined;
pub use blackmar_diemer_gambit_declined::BLACKMAR_DIEMER_GAMBIT_DECLINED;
pub mod blumenfeld_countergambit;
pub use blumenfeld_countergambit::BLUMENFELD_COUNTERGAMBIT;
pub mod blumenfeld_countergambit_accepted;
pub use blumenfeld_countergambit_accepted::BLUMENFELD_COUNTERGAMBIT_ACCEPTED;
pub mod bogo_indian_defense;
pub use bogo_indian_defense::BOGO_INDIAN_DEFENSE;
pub mod bongcloud_attack;
pub use bongcloud_attack::BONGCLOUD_ATTACK;
pub mod borg_defense;
pub use borg_defense::BORG_DEFENSE;
pub mod canard_opening;
pub use canard_opening::CANARD_OPENING;
pub mod caro_kann_defense;
pub use caro_kann_defense::CARO_KANN_DEFENSE;
pub mod carr_defense;
pub use carr_defense::CARR_DEFENSE;
pub mod catalan_opening;
pub use catalan_opening::CATALAN_OPENING;
pub mod center_game;
pub use center_game::CENTER_GAME;
pub mod center_game_accepted;
pub use center_game_accepted::CENTER_GAME_ACCEPTED;
pub mod clemenz_opening;
pub use clemenz_opening::CLEMENZ_OPENING;
pub mod colle_system;
pub use colle_system::COLLE_SYSTEM;
pub mod creepy_crawly_formation;
pub use creepy_crawly_formation::CREEPY_CRAWLY_FORMATION;
pub mod czech_defense;
pub use czech_defense::CZECH_DEFENSE;
pub mod danish_gambit;
pub use danish_gambit::DANISH_GAMBIT;
pub mod danish_gambit_accepted;
pub use danish_gambit_accepted::DANISH_GAMBIT_ACCEPTED;
pub mod danish_gambit_declined;
pub use danish_gambit_declined::DANISH_GAMBIT_DECLINED;
pub mod dresden_opening;
pub use dresden_opening::DRESDEN_OPENING;
pub mod duras_gambit;
pub use duras_gambit::DURAS_GAMBIT;
pub mod dutch_defense;
pub use dutch_defense::DUTCH_DEFENSE;
pub mod dory_defense;
pub use dory_defense::DORY_DEFENSE;
pub mod east_indian_defense;
pub use east_indian_defense::EAST_INDIAN_DEFENSE;
pub mod elephant_gambit;
pub use elephant_gambit::ELEPHANT_GAMBIT;
pub mod english_defense;
pub use english_defense::ENGLISH_DEFENSE;
pub mod english_opening;
pub use english_opening::ENGLISH_OPENING;
pub mod english_orangutan;
pub use english_orangutan::ENGLISH_ORANGUTAN;
pub mod englund_gambit;
pub use englund_gambit::ENGLUND_GAMBIT;
pub mod englund_gambit_declined;
pub use englund_gambit_declined::ENGLUND_GAMBIT_DECLINED;
pub mod formation;
pub use formation::FORMATION;
pub mod four_knights_game;
pub use four_knights_game::FOUR_KNIGHTS_GAME;
pub mod french_defense;
pub use french_defense::FRENCH_DEFENSE;
pub mod fried_fox_defense;
pub use fried_fox_defense::FRIED_FOX_DEFENSE;
pub mod global_opening;
pub use global_opening::GLOBAL_OPENING;
pub mod goldsmith_defense;
pub use goldsmith_defense::GOLDSMITH_DEFENSE;
pub mod grob_opening;
pub use grob_opening::GROB_OPENING;
pub mod grunfeld_defense;
pub use grunfeld_defense::GRUNFELD_DEFENSE;
pub mod gunderam_defense;
pub use gunderam_defense::GUNDERAM_DEFENSE;
pub mod hippopotamus_defense;
pub use hippopotamus_defense::HIPPOPOTAMUS_DEFENSE;
pub mod horwitz_defense;
pub use horwitz_defense::HORWITZ_DEFENSE;
pub mod hungarian_opening;
pub use hungarian_opening::HUNGARIAN_OPENING;
pub mod indian_defense;
pub use indian_defense::INDIAN_DEFENSE;
pub mod irish_gambit;
pub use irish_gambit::IRISH_GAMBIT;
pub mod italian_game;
pub use italian_game::ITALIAN_GAME;
pub mod kangaroo_defense;
pub use kangaroo_defense::KANGAROO_DEFENSE;
pub mod kings_gambit;
pub use kings_gambit::KINGS_GAMBIT;
pub mod kings_gambit_accepted;
pub use kings_gambit_accepted::KINGS_GAMBIT_ACCEPTED;
pub mod kings_gambit_declined;
pub use kings_gambit_declined::KINGS_GAMBIT_DECLINED;
pub mod kings_indian_attack;
pub use kings_indian_attack::KINGS_INDIAN_ATTACK;
pub mod kings_indian_defense;
pub use kings_indian_defense::KINGS_INDIAN_DEFENSE;
pub mod kings_knight_opening;
pub use kings_knight_opening::KINGS_KNIGHT_OPENING;
pub mod kings_pawn_game;
pub use kings_pawn_game::KINGS_PAWN_GAME;
pub mod kings_pawn_opening;
pub use kings_pawn_opening::KINGS_PAWN_OPENING;
pub mod kadas_opening;
pub use kadas_opening::KADAS_OPENING;
pub mod lasker_simul_special;
pub use lasker_simul_special::LASKER_SIMUL_SPECIAL;
pub mod latvian_gambit;
pub use latvian_gambit::LATVIAN_GAMBIT;
pub mod latvian_gambit_accepted;
pub use latvian_gambit_accepted::LATVIAN_GAMBIT_ACCEPTED;
pub mod lemming_defense;
pub use lemming_defense::LEMMING_DEFENSE;
pub mod lion_defense;
pub use lion_defense::LION_DEFENSE;
pub mod london_system;
pub use london_system::LONDON_SYSTEM;
pub mod marienbad_system;
pub use marienbad_system::MARIENBAD_SYSTEM;
pub mod mexican_defense;
pub use mexican_defense::MEXICAN_DEFENSE;
pub mod mieses_opening;
pub use mieses_opening::MIESES_OPENING;
pub mod mikenas_defense;
pub use mikenas_defense::MIKENAS_DEFENSE;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
pub mod montevideo_defense;
pub use montevideo_defense::MONTEVIDEO_DEFENSE;
pub mod neo_grunfeld_defense;
pub use neo_grunfeld_defense::NEO_GRUNFELD_DEFENSE;
pub mod nimzo_indian_defense;
pub use nimzo_indian_defense::NIMZO_INDIAN_DEFENSE;
pub mod nimzo_larsen_attack;
pub use nimzo_larsen_attack::NIMZO_LARSEN_ATTACK;
pub mod nimzowitsch_defense;
pub use nimzowitsch_defense::NIMZOWITSCH_DEFENSE;
pub mod old_indian_defense;
pub use old_indian_defense::OLD_INDIAN_DEFENSE;
pub mod owen_defense;
pub use owen_defense::OWEN_DEFENSE;
pub mod paleface_attack;
pub use paleface_attack::PALEFACE_ATTACK;
pub mod petrovs_defense;
pub use petrovs_defense::PETROVS_DEFENSE;
pub mod philidor_defense;
pub use philidor_defense::PHILIDOR_DEFENSE;
pub mod pirc_defense;
pub use pirc_defense::PIRC_DEFENSE;
pub mod polish_defense;
pub use polish_defense::POLISH_DEFENSE;
pub mod polish_opening;
pub use polish_opening::POLISH_OPENING;
pub mod ponziani_opening;
pub use ponziani_opening::PONZIANI_OPENING;
pub mod portuguese_opening;
pub use portuguese_opening::PORTUGUESE_OPENING;
pub mod pseudo_queens_indian_defense;
pub use pseudo_queens_indian_defense::PSEUDO_QUEENS_INDIAN_DEFENSE;
pub mod pterodactyl_defense;
pub use pterodactyl_defense::PTERODACTYL_DEFENSE;
pub mod queens_gambit;
pub use queens_gambit::QUEENS_GAMBIT;
pub mod queens_gambit_accepted;
pub use queens_gambit_accepted::QUEENS_GAMBIT_ACCEPTED;
pub mod queens_gambit_declined;
pub use queens_gambit_declined::QUEENS_GAMBIT_DECLINED;
pub mod queens_indian_accelerated;
pub use queens_indian_accelerated::QUEENS_INDIAN_ACCELERATED;
pub mod queens_indian_defense;
pub use queens_indian_defense::QUEENS_INDIAN_DEFENSE;
pub mod queens_pawn;
pub use queens_pawn::QUEENS_PAWN;
pub mod queens_pawn_game;
pub use queens_pawn_game::QUEENS_PAWN_GAME;
pub mod rapport_jobava_system;
pub use rapport_jobava_system::RAPPORT_JOBAVA_SYSTEM;
pub mod rat_defense;
pub use rat_defense::RAT_DEFENSE;
pub mod richter_veresov_attack;
pub use richter_veresov_attack::RICHTER_VERESOV_ATTACK;
pub mod robatsch_defense;
pub use robatsch_defense::ROBATSCH_DEFENSE;
pub mod rubinstein_opening;
pub use rubinstein_opening::RUBINSTEIN_OPENING;
pub mod ruy_lopez;
pub use ruy_lopez::RUY_LOPEZ;
pub mod reti_opening;
pub use reti_opening::RETI_OPENING;
pub mod saragossa_opening;
pub use saragossa_opening::SARAGOSSA_OPENING;
pub mod scandinavian_defense;
pub use scandinavian_defense::SCANDINAVIAN_DEFENSE;
pub mod scotch_game;
pub use scotch_game::SCOTCH_GAME;
pub mod semi_slav_defense;
pub use semi_slav_defense::SEMI_SLAV_DEFENSE;
pub mod semi_slav_defense_accepted;
pub use semi_slav_defense_accepted::SEMI_SLAV_DEFENSE_ACCEPTED;
pub mod sicilian_defense;
pub use sicilian_defense::SICILIAN_DEFENSE;
pub mod slav_defense;
pub use slav_defense::SLAV_DEFENSE;
pub mod slav_indian;
pub use slav_indian::SLAV_INDIAN;
pub mod sodium_attack;
pub use sodium_attack::SODIUM_ATTACK;
pub mod st_george_defense;
pub use st_george_defense::ST_GEORGE_DEFENSE;
pub mod tarrasch_defense;
pub use tarrasch_defense::TARRASCH_DEFENSE;
pub mod three_knights_opening;
pub use three_knights_opening::THREE_KNIGHTS_OPENING;
pub mod torre_attack;
pub use torre_attack::TORRE_ATTACK;
pub mod trompowsky_attack;
pub use trompowsky_attack::TROMPOWSKY_ATTACK;
pub mod valencia_opening;
pub use valencia_opening::VALENCIA_OPENING;
pub mod van_geet_opening;
pub use van_geet_opening::VAN_GEET_OPENING;
pub mod vant_kruijs_opening;
pub use vant_kruijs_opening::VANT_KRUIJS_OPENING;
pub mod vienna_gambit;
pub use vienna_gambit::VIENNA_GAMBIT;
pub mod vienna_game;
pub use vienna_game::VIENNA_GAME;
pub mod vulture_defense;
pub use vulture_defense::VULTURE_DEFENSE;
pub mod wade_defense;
pub use wade_defense::WADE_DEFENSE;
pub mod ware_defense;
pub use ware_defense::WARE_DEFENSE;
pub mod ware_opening;
pub use ware_opening::WARE_OPENING;
pub mod yusupov_rubinstein_system;
pub use yusupov_rubinstein_system::YUSUPOV_RUBINSTEIN_SYSTEM;
pub mod zaire_defense;
pub use zaire_defense::ZAIRE_DEFENSE;
pub mod zukertort_defense;
pub use zukertort_defense::ZUKERTORT_DEFENSE;
pub mod zukertort_opening;
pub use zukertort_opening::ZUKERTORT_OPENING;

/// The total amount of variations.
pub const VARIATION_COUNT: usize = 3223;

/// All root variations.
pub static ALL: [&Variation; 140] = [
    &ALEKHINE_DEFENSE,
    &AMAR_OPENING,
    &AMAZON_ATTACK,
    &AMSTERDAM_ATTACK,
    &ANDERSSENS_OPENING,
    &AUSTRALIAN_DEFENSE,
    &BARNES_DEFENSE,
    &BARNES_OPENING,
    &BENKO_GAMBIT,
    &BENKO_GAMBIT_ACCEPTED,
    &BENKO_GAMBIT_DECLINED,
    &BENONI_DEFENSE,
    &BIRD_OPENING,
    &BISHOPS_OPENING,
    &BLACKMAR_DIEMER_GAMBIT,
    &BLACKMAR_DIEMER_GAMBIT_ACCEPTED,
    &BLACKMAR_DIEMER_GAMBIT_DECLINED,
    &BLUMENFELD_COUNTERGAMBIT,
    &BLUMENFELD_COUNTERGAMBIT_ACCEPTED,
    &BOGO_INDIAN_DEFENSE,
    &BONGCLOUD_ATTACK,
    &BORG_DEFENSE,
    &CANARD_OPENING,
    &CARO_KANN_DEFENSE,
    &CARR_DEFENSE,
    &CATALAN_OPENING,
    &CENTER_GAME,
    &CENTER_GAME_ACCEPTED,
    &CLEMENZ_OPENING,
    &COLLE_SYSTEM,
    &CREEPY_CRAWLY_FORMATION,
    &CZECH_DEFENSE,
    &DANISH_GAMBIT,
    &DANISH_GAMBIT_ACCEPTED,
    &DANISH_GAMBIT_DECLINED,
    &DRESDEN_OPENING,
    &DURAS_GAMBIT,
    &DUTCH_DEFENSE,
    &DORY_DEFENSE,
    &EAST_INDIAN_DEFENSE,
    &ELEPHANT_GAMBIT,
    &ENGLISH_DEFENSE,
    &ENGLISH_OPENING,
    &ENGLISH_ORANGUTAN,
    &ENGLUND_GAMBIT,
    &ENGLUND_GAMBIT_DECLINED,
    &FORMATION,
    &FOUR_KNIGHTS_GAME,
    &FRENCH_DEFENSE,
    &FRIED_FOX_DEFENSE,
    &GLOBAL_OPENING,
    &GOLDSMITH_DEFENSE,
    &GROB_OPENING,
    &GRUNFELD_DEFENSE,
    &GUNDERAM_DEFENSE,
    &HIPPOPOTAMUS_DEFENSE,
    &HORWITZ_DEFENSE,
    &HUNGARIAN_OPENING,
    &INDIAN_DEFENSE,
    &IRISH_GAMBIT,
    &ITALIAN_GAME,
    &KANGAROO_DEFENSE,
    &KINGS_GAMBIT,
    &KINGS_GAMBIT_ACCEPTED,
    &KINGS_GAMBIT_DECLINED,
    &KINGS_INDIAN_ATTACK,
    &KINGS_INDIAN_DEFENSE,
    &KINGS_KNIGHT_OPENING,
    &KINGS_PAWN_GAME,
    &KINGS_PAWN_OPENING,
    &KADAS_OPENING,
    &LASKER_SIMUL_SPECIAL,
    &LATVIAN_GAMBIT,
    &LATVIAN_GAMBIT_ACCEPTED,
    &LEMMING_DEFENSE,
    &LION_DEFENSE,
    &LONDON_SYSTEM,
    &MARIENBAD_SYSTEM,
    &MEXICAN_DEFENSE,
    &MIESES_OPENING,
    &MIKENAS_DEFENSE,
    &MODERN_DEFENSE,
    &MONTEVIDEO_DEFENSE,
    &NEO_GRUNFELD_DEFENSE,
    &NIMZO_INDIAN_DEFENSE,
    &NIMZO_LARSEN_ATTACK,
    &NIMZOWITSCH_DEFENSE,
    &OLD_INDIAN_DEFENSE,
    &OWEN_DEFENSE,
    &PALEFACE_ATTACK,
    &PETROVS_DEFENSE,
    &PHILIDOR_DEFENSE,
    &PIRC_DEFENSE,
    &POLISH_DEFENSE,
    &POLISH_OPENING,
    &PONZIANI_OPENING,
    &PORTUGUESE_OPENING,
    &PSEUDO_QUEENS_INDIAN_DEFENSE,
    &PTERODACTYL_DEFENSE,
    &QUEENS_GAMBIT,
    &QUEENS_GAMBIT_ACCEPTED,
    &QUEENS_GAMBIT_DECLINED,
    &QUEENS_INDIAN_ACCELERATED,
    &QUEENS_INDIAN_DEFENSE,
    &QUEENS_PAWN,
    &QUEENS_PAWN_GAME,
    &RAPPORT_JOBAVA_SYSTEM,
    &RAT_DEFENSE,
    &RICHTER_VERESOV_ATTACK,
    &ROBATSCH_DEFENSE,
    &RUBINSTEIN_OPENING,
    &RUY_LOPEZ,
    &RETI_OPENING,
    &SARAGOSSA_OPENING,
    &SCANDINAVIAN_DEFENSE,
    &SCOTCH_GAME,
    &SEMI_SLAV_DEFENSE,
    &SEMI_SLAV_DEFENSE_ACCEPTED,
    &SICILIAN_DEFENSE,
    &SLAV_DEFENSE,
    &SLAV_INDIAN,
    &SODIUM_ATTACK,
    &ST_GEORGE_DEFENSE,
    &TARRASCH_DEFENSE,
    &THREE_KNIGHTS_OPENING,
    &TORRE_ATTACK,
    &TROMPOWSKY_ATTACK,
    &VALENCIA_OPENING,
    &VAN_GEET_OPENING,
    &VANT_KRUIJS_OPENING,
    &VIENNA_GAMBIT,
    &VIENNA_GAME,
    &VULTURE_DEFENSE,
    &WADE_DEFENSE,
    &WARE_DEFENSE,
    &WARE_OPENING,
    &YUSUPOV_RUBINSTEIN_SYSTEM,
    &ZAIRE_DEFENSE,
    &ZUKERTORT_DEFENSE,
    &ZUKERTORT_OPENING,
];
