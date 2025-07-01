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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
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
# use reco::book::italian_game::EVANS_GAMBIT;
assert_eq!(EVANS_GAMBIT.original_name(), "Italian Game: Evans Gambit");
```"#
)]
pub static EVANS_GAMBIT: Variation = Variation {
    name: "Evans Gambit",
    parent: Some(&super::ITALIAN_GAME),
    variations: &[
        &ALAPIN_STEINITZ_VARIATION,
        &ANDERSSEN_DEFENSE,
        &ANDERSSEN_VARIATION,
        &BRONSTEIN_DEFENSE,
        &COMPROMISED_DEFENSE,
        &DUFRESNE_DEFENSE,
        &FONTAINE_COUNTERGAMBIT,
        &FRASER_ATTACK,
        &FRASER_MORTIMER_ATTACK,
        &GORING_ATTACK,
        &HARDING_VARIATION,
        &HEIN_COUNTERGAMBIT,
        &JOHNER_DEFENSE,
        &LAROCHE_VARIATION,
        &LASKER_DEFENSE,
        &LEONHARDT_COUNTERGAMBIT,
        &LEVENFISH_VARIATION,
        &MAIN_LINE,
        &MAYET_DEFENSE,
        &MC_DONNELL_DEFENSE,
        &MIESES_DEFENSE,
        &MORPHY_ATTACK,
        &MORTIMER_EVANS_GAMBIT,
        &PAULSEN_VARIATION,
        &PIERCE_DEFENSE,
        &RICHARDSON_ATTACK,
        &SANDERS_ALAPIN_VARIATION,
        &SLOW_VARIATION,
        &SOKOLSKY_VARIATION,
        &STEINITZ_VARIATION,
        &STONE_WARE_VARIATION,
        &TARTAKOWER_ATTACK,
        &ULVESTAD_VARIATION,
        &WALLER_ATTACK,
    ],
    lines: &[
        Line {
            parent: &EVANS_GAMBIT,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<5>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
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
            parent: &EVANS_GAMBIT,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<5>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C5,
                    capture: Some(Pawn),
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E5,
                    capture: Some(Pawn),
                    to: Square::D4,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C3,
                    capture: Some(Pawn),
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C5,
                    capture: None,
                    to: Square::B6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C8,
                    capture: None,
                    to: Square::G4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
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
            parent: &EVANS_GAMBIT,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<5>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C5,
                    capture: Some(Pawn),
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::A5,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
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
pub mod alapin_steinitz_variation;
pub use alapin_steinitz_variation::ALAPIN_STEINITZ_VARIATION;
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod bronstein_defense;
pub use bronstein_defense::BRONSTEIN_DEFENSE;
pub mod compromised_defense;
pub use compromised_defense::COMPROMISED_DEFENSE;
pub mod dufresne_defense;
pub use dufresne_defense::DUFRESNE_DEFENSE;
pub mod fontaine_countergambit;
pub use fontaine_countergambit::FONTAINE_COUNTERGAMBIT;
pub mod fraser_attack;
pub use fraser_attack::FRASER_ATTACK;
pub mod fraser_mortimer_attack;
pub use fraser_mortimer_attack::FRASER_MORTIMER_ATTACK;
pub mod goring_attack;
pub use goring_attack::GORING_ATTACK;
pub mod harding_variation;
pub use harding_variation::HARDING_VARIATION;
pub mod hein_countergambit;
pub use hein_countergambit::HEIN_COUNTERGAMBIT;
pub mod johner_defense;
pub use johner_defense::JOHNER_DEFENSE;
pub mod laroche_variation;
pub use laroche_variation::LAROCHE_VARIATION;
pub mod lasker_defense;
pub use lasker_defense::LASKER_DEFENSE;
pub mod leonhardt_countergambit;
pub use leonhardt_countergambit::LEONHARDT_COUNTERGAMBIT;
pub mod levenfish_variation;
pub use levenfish_variation::LEVENFISH_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod mayet_defense;
pub use mayet_defense::MAYET_DEFENSE;
pub mod mc_donnell_defense;
pub use mc_donnell_defense::MC_DONNELL_DEFENSE;
pub mod mieses_defense;
pub use mieses_defense::MIESES_DEFENSE;
pub mod morphy_attack;
pub use morphy_attack::MORPHY_ATTACK;
pub mod mortimer_evans_gambit;
pub use mortimer_evans_gambit::MORTIMER_EVANS_GAMBIT;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod pierce_defense;
pub use pierce_defense::PIERCE_DEFENSE;
pub mod richardson_attack;
pub use richardson_attack::RICHARDSON_ATTACK;
pub mod sanders_alapin_variation;
pub use sanders_alapin_variation::SANDERS_ALAPIN_VARIATION;
pub mod slow_variation;
pub use slow_variation::SLOW_VARIATION;
pub mod sokolsky_variation;
pub use sokolsky_variation::SOKOLSKY_VARIATION;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod stone_ware_variation;
pub use stone_ware_variation::STONE_WARE_VARIATION;
pub mod tartakower_attack;
pub use tartakower_attack::TARTAKOWER_ATTACK;
pub mod ulvestad_variation;
pub use ulvestad_variation::ULVESTAD_VARIATION;
pub mod waller_attack;
pub use waller_attack::WALLER_ATTACK;
