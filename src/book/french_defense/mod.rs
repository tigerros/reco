#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::FRENCH_DEFENSE;
assert_eq!(FRENCH_DEFENSE.original_name(), "French Defense");
```"#)]
pub static FRENCH_DEFENSE: Variation = Variation {
    name: "French Defense",
    variations: &[&STEINITZ_ATTACK,
&WINAWER_VARIATION,
&NORMAL_VARIATION,
&BAEUERLE_GAMBIT,
&LA_BOURDONNAIS_VARIATION,
&RUBINSTEIN_VARIATION,
&MARSHALL_GAMBIT,
&FRANCO_SICILIAN_DEFENSE,
&RETI_SPIELMANN_ATTACK,
&ALAPIN_GAMBIT,
&BURN_VARIATION,
&DIEMER_DUHM_GAMBIT,
&KINGS_INDIAN_ATTACK,
&ORTHOSCHNAPP_GAMBIT,
&WING_GAMBIT,
&CHIGORIN_VARIATION,
&MAC_CUTCHEON_VARIATION,
&BANZAI_LEONG_GAMBIT,
&KNIGHT_VARIATION,
&HOFFMANN_GAMBIT,
&EXCHANGE_VARIATION,
&ADVANCE_VARIATION,
&GUIMARD_VARIATION,
&HORWITZ_ATTACK,
&PAULSEN_VARIATION,
&BIRD_INVITATION,
&STEINITZ_VARIATION,
&MORPHY_GAMBIT,
&ALEKHINE_CHATARD_ATTACK,
&HENNEBERGER_VARIATION,
&PERSEUS_GAMBIT,
&SCHLECHTER_VARIATION,
&PELIKAN_VARIATION,
&REVERSED_PHILIDOR_FORMATION,
&ST_GEORGE_DEFENSE,
&CARLSON_GAMBIT,
&FRANCO_HIVA_GAMBIT_ACCEPTED,
&STEINER_VARIATION,
&DIEMER_DUHM_GAMBIT_ACCEPTED,
&MEDITERRANEAN_DEFENSE,
&TWO_KNIGHTS_VARIATION,
&TARRASCH_VARIATION,
&HECHT_REEFSCHLAGER_VARIATION,
&CLASSICAL_VARIATION,
&QUEENS_KNIGHT,
&FRANCO_HIVA_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<0>()
    },
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
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65038346568656640),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18439724825837568000),
                white: Bitboard(402712575)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<0>()
    },
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
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111888387840),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441976591291514880),
                white: Bitboard(268496895)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod steinitz_attack;
pub use steinitz_attack::STEINITZ_ATTACK;
pub mod winawer_variation;
pub use winawer_variation::WINAWER_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod baeuerle_gambit;
pub use baeuerle_gambit::BAEUERLE_GAMBIT;
pub mod la_bourdonnais_variation;
pub use la_bourdonnais_variation::LA_BOURDONNAIS_VARIATION;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod franco_sicilian_defense;
pub use franco_sicilian_defense::FRANCO_SICILIAN_DEFENSE;
pub mod reti_spielmann_attack;
pub use reti_spielmann_attack::RETI_SPIELMANN_ATTACK;
pub mod alapin_gambit;
pub use alapin_gambit::ALAPIN_GAMBIT;
pub mod burn_variation;
pub use burn_variation::BURN_VARIATION;
pub mod diemer_duhm_gambit;
pub use diemer_duhm_gambit::DIEMER_DUHM_GAMBIT;
pub mod kings_indian_attack;
pub use kings_indian_attack::KINGS_INDIAN_ATTACK;
pub mod orthoschnapp_gambit;
pub use orthoschnapp_gambit::ORTHOSCHNAPP_GAMBIT;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod chigorin_variation;
pub use chigorin_variation::CHIGORIN_VARIATION;
pub mod mac_cutcheon_variation;
pub use mac_cutcheon_variation::MAC_CUTCHEON_VARIATION;
pub mod banzai_leong_gambit;
pub use banzai_leong_gambit::BANZAI_LEONG_GAMBIT;
pub mod knight_variation;
pub use knight_variation::KNIGHT_VARIATION;
pub mod hoffmann_gambit;
pub use hoffmann_gambit::HOFFMANN_GAMBIT;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod guimard_variation;
pub use guimard_variation::GUIMARD_VARIATION;
pub mod horwitz_attack;
pub use horwitz_attack::HORWITZ_ATTACK;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod bird_invitation;
pub use bird_invitation::BIRD_INVITATION;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod morphy_gambit;
pub use morphy_gambit::MORPHY_GAMBIT;
pub mod alekhine_chatard_attack;
pub use alekhine_chatard_attack::ALEKHINE_CHATARD_ATTACK;
pub mod henneberger_variation;
pub use henneberger_variation::HENNEBERGER_VARIATION;
pub mod perseus_gambit;
pub use perseus_gambit::PERSEUS_GAMBIT;
pub mod schlechter_variation;
pub use schlechter_variation::SCHLECHTER_VARIATION;
pub mod pelikan_variation;
pub use pelikan_variation::PELIKAN_VARIATION;
pub mod reversed_philidor_formation;
pub use reversed_philidor_formation::REVERSED_PHILIDOR_FORMATION;
pub mod st_george_defense;
pub use st_george_defense::ST_GEORGE_DEFENSE;
pub mod carlson_gambit;
pub use carlson_gambit::CARLSON_GAMBIT;
pub mod franco_hiva_gambit_accepted;
pub use franco_hiva_gambit_accepted::FRANCO_HIVA_GAMBIT_ACCEPTED;
pub mod steiner_variation;
pub use steiner_variation::STEINER_VARIATION;
pub mod diemer_duhm_gambit_accepted;
pub use diemer_duhm_gambit_accepted::DIEMER_DUHM_GAMBIT_ACCEPTED;
pub mod mediterranean_defense;
pub use mediterranean_defense::MEDITERRANEAN_DEFENSE;
pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
pub mod tarrasch_variation;
pub use tarrasch_variation::TARRASCH_VARIATION;
pub mod hecht_reefschlager_variation;
pub use hecht_reefschlager_variation::HECHT_REEFSCHLAGER_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod queens_knight;
pub use queens_knight::QUEENS_KNIGHT;
pub mod franco_hiva_gambit;
pub use franco_hiva_gambit::FRANCO_HIVA_GAMBIT;
