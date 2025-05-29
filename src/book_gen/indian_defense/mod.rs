#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
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
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::INDIAN_DEFENSE;
assert_eq!(INDIAN_DEFENSE.original_name(), "Indian Defense");
```"#
)]
pub static INDIAN_DEFENSE: Variation = Variation {
    name: "Indian Defense",
    parent: None,
    variations: &[
        &OMEGA_GAMBIT,
        &MEDUSA_GAMBIT,
        &COLLE_SYSTEM,
        &BUDAPEST_DEFENSE,
        &PRZEPIORKA_VARIATION,
        &DZINDZI_INDIAN_DEFENSE,
        &DEVIN_GAMBIT,
        &WEST_INDIAN_DEFENSE,
        &ANTI_GRUNFELD,
        &PALEFACE_ATTACK,
        &GIBBINS_WEIDENHAGEN_GAMBIT,
        &KINGS_INDIAN_VARIATION,
        &PSEUDO_BENKO,
        &WADE_TARTAKOWER_DEFENSE,
        &PAWN_PUSH_VARIATION,
        &TARTAKOWER_ATTACK,
        &KNIGHTS_VARIATION,
        &LONDON_SYSTEM,
        &NORMAL_VARIATION,
        &REVERSED_CHIGORIN_DEFENSE,
        &SPIELMANN_INDIAN,
        &ANTI_NIMZO_INDIAN,
        &SEIRAWAN_ATTACK,
        &GEDULT_ATTACK,
        &LAZARD_GAMBIT,
        &SCHNEPPER_GAMBIT,
        &DORY_INDIAN,
        &CZECH_INDIAN,
        &POLISH_VARIATION,
        &PYRENEES_GAMBIT,
        &GIBBINS_WEIDENHAGEN_GAMBIT_ACCEPTED,
        &MADDIGAN_GAMBIT,
    ],
    lines: &[
        Line {
            parent: &INDIAN_DEFENSE,
            code: Code {
                volume: Volume::E,
                category: Category::new_static::<0>(),
            },
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
            setup: Setup {
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
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &INDIAN_DEFENSE,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<45>(),
            },
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
            setup: Setup {
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
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
    ],
};
pub mod omega_gambit;
pub use omega_gambit::OMEGA_GAMBIT;
pub mod medusa_gambit;
pub use medusa_gambit::MEDUSA_GAMBIT;
pub mod colle_system;
pub use colle_system::COLLE_SYSTEM;
pub mod budapest_defense;
pub use budapest_defense::BUDAPEST_DEFENSE;
pub mod przepiorka_variation;
pub use przepiorka_variation::PRZEPIORKA_VARIATION;
pub mod dzindzi_indian_defense;
pub use dzindzi_indian_defense::DZINDZI_INDIAN_DEFENSE;
pub mod devin_gambit;
pub use devin_gambit::DEVIN_GAMBIT;
pub mod west_indian_defense;
pub use west_indian_defense::WEST_INDIAN_DEFENSE;
pub mod anti_grunfeld;
pub use anti_grunfeld::ANTI_GRUNFELD;
pub mod paleface_attack;
pub use paleface_attack::PALEFACE_ATTACK;
pub mod gibbins_weidenhagen_gambit;
pub use gibbins_weidenhagen_gambit::GIBBINS_WEIDENHAGEN_GAMBIT;
pub mod kings_indian_variation;
pub use kings_indian_variation::KINGS_INDIAN_VARIATION;
pub mod pseudo_benko;
pub use pseudo_benko::PSEUDO_BENKO;
pub mod wade_tartakower_defense;
pub use wade_tartakower_defense::WADE_TARTAKOWER_DEFENSE;
pub mod pawn_push_variation;
pub use pawn_push_variation::PAWN_PUSH_VARIATION;
pub mod tartakower_attack;
pub use tartakower_attack::TARTAKOWER_ATTACK;
pub mod knights_variation;
pub use knights_variation::KNIGHTS_VARIATION;
pub mod london_system;
pub use london_system::LONDON_SYSTEM;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod reversed_chigorin_defense;
pub use reversed_chigorin_defense::REVERSED_CHIGORIN_DEFENSE;
pub mod spielmann_indian;
pub use spielmann_indian::SPIELMANN_INDIAN;
pub mod anti_nimzo_indian;
pub use anti_nimzo_indian::ANTI_NIMZO_INDIAN;
pub mod seirawan_attack;
pub use seirawan_attack::SEIRAWAN_ATTACK;
pub mod gedult_attack;
pub use gedult_attack::GEDULT_ATTACK;
pub mod lazard_gambit;
pub use lazard_gambit::LAZARD_GAMBIT;
pub mod schnepper_gambit;
pub use schnepper_gambit::SCHNEPPER_GAMBIT;
pub mod dory_indian;
pub use dory_indian::DORY_INDIAN;
pub mod czech_indian;
pub use czech_indian::CZECH_INDIAN;
pub mod polish_variation;
pub use polish_variation::POLISH_VARIATION;
pub mod pyrenees_gambit;
pub use pyrenees_gambit::PYRENEES_GAMBIT;
pub mod gibbins_weidenhagen_gambit_accepted;
pub use gibbins_weidenhagen_gambit_accepted::GIBBINS_WEIDENHAGEN_GAMBIT_ACCEPTED;
pub mod maddigan_gambit;
pub use maddigan_gambit::MADDIGAN_GAMBIT;
