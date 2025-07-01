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
# use reco::book::ruy_lopez::MORPHY_DEFENSE;
assert_eq!(MORPHY_DEFENSE.original_name(), "Ruy Lopez: Morphy Defense");
```"#
)]
pub static MORPHY_DEFENSE: Variation = Variation {
    name: "Morphy Defense",
    parent: Some(&super::RUY_LOPEZ),
    variations: &[
        &ALAPINS_DEFENSE_DEFERRED,
        &ANDERSSEN_VARIATION,
        &ARKHANGELSK_VARIATION,
        &BAYREUTH_VARIATION,
        &CARO_VARIATION,
        &CLASSICAL_DEFENSE_DEFERRED,
        &COZIO_DEFENSE,
        &DURAS_VARIATION,
        &FIANCHETTO_DEFENSE_DEFERRED,
        &GRAZ_VARIATION,
        &JAFFE_GAMBIT,
        &MACKENZIE_VARIATION,
        &MODERN_STEINITZ_DEFENSE,
        &NEO_ARKHANGELSK_VARIATION,
        &NORWEGIAN_VARIATION,
        &SCHLIEMANN_DEFENSE_DEFERRED,
        &STEINITZ_DEFERRED,
        &TARRASCH_VARIATION,
        &TARTAKOWER_VARIATION,
        &WING_ATTACK,
        &WORMALD_ATTACK,
    ],
    lines: &[
        Line {
            parent: &MORPHY_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<7>()),
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
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66992212956737280),
                        knight: Bitboard(4611690416475996162),
                        bishop: Bitboard(2594073393955340292),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18297567902330519552),
                        white: Bitboard(8860528543),
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
            parent: &MORPHY_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<7>()),
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
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66992212956737280),
                        knight: Bitboard(39582420697090),
                        bishop: Bitboard(2594073385382182916),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13685917068275220480),
                        white: Bitboard(287371119),
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
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 3,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
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
            parent: &MORPHY_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<7>()),
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
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::A4,
                    capture: None,
                    to: Square::B3,
                    promotion: None,
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
                        pawn: Bitboard(64186267872587520),
                        knight: Bitboard(39582420697090),
                        bishop: Bitboard(2594073385365536772),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13683111123191070720),
                        white: Bitboard(270724975),
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
pub mod alapins_defense_deferred;
pub use alapins_defense_deferred::ALAPINS_DEFENSE_DEFERRED;
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod arkhangelsk_variation;
pub use arkhangelsk_variation::ARKHANGELSK_VARIATION;
pub mod bayreuth_variation;
pub use bayreuth_variation::BAYREUTH_VARIATION;
pub mod caro_variation;
pub use caro_variation::CARO_VARIATION;
pub mod classical_defense_deferred;
pub use classical_defense_deferred::CLASSICAL_DEFENSE_DEFERRED;
pub mod cozio_defense;
pub use cozio_defense::COZIO_DEFENSE;
pub mod duras_variation;
pub use duras_variation::DURAS_VARIATION;
pub mod fianchetto_defense_deferred;
pub use fianchetto_defense_deferred::FIANCHETTO_DEFENSE_DEFERRED;
pub mod graz_variation;
pub use graz_variation::GRAZ_VARIATION;
pub mod jaffe_gambit;
pub use jaffe_gambit::JAFFE_GAMBIT;
pub mod mackenzie_variation;
pub use mackenzie_variation::MACKENZIE_VARIATION;
pub mod modern_steinitz_defense;
pub use modern_steinitz_defense::MODERN_STEINITZ_DEFENSE;
pub mod neo_arkhangelsk_variation;
pub use neo_arkhangelsk_variation::NEO_ARKHANGELSK_VARIATION;
pub mod norwegian_variation;
pub use norwegian_variation::NORWEGIAN_VARIATION;
pub mod schliemann_defense_deferred;
pub use schliemann_defense_deferred::SCHLIEMANN_DEFENSE_DEFERRED;
pub mod steinitz_deferred;
pub use steinitz_deferred::STEINITZ_DEFERRED;
pub mod tarrasch_variation;
pub use tarrasch_variation::TARRASCH_VARIATION;
pub mod tartakower_variation;
pub use tartakower_variation::TARTAKOWER_VARIATION;
pub mod wing_attack;
pub use wing_attack::WING_ATTACK;
pub mod wormald_attack;
pub use wormald_attack::WORMALD_ATTACK;
