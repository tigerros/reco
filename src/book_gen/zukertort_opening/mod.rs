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
# use reco::book::ZUKERTORT_OPENING;
assert_eq!(ZUKERTORT_OPENING.original_name(), "Zukertort Opening");
```"#
)]
pub static ZUKERTORT_OPENING: Variation = Variation {
    name: "Zukertort Opening",
    parent: None,
    variations: &[
        &QUEENSIDE_FIANCHETTO_VARIATION,
        &POLISH_DEFENSE,
        &REGINA_NU_GAMBIT,
        &THE_POTATO,
        &REVERSED_GRUNFELD,
        &SHABALOV_GAMBIT,
        &SPEELSMET_GAMBIT,
        &THE_WALRUS,
        &ST_GEORGE_DEFENSE,
        &LISITSYN_GAMBIT_DEFERRED,
        &LEMBERGER_GAMBIT,
        &SICILIAN_INVITATION,
        &NIMZO_LARSEN_VARIATION,
        &AMPEL_VARIATION,
        &DOUBLE_FIANCHETTO_ATTACK,
        &ROSS_GAMBIT,
        &BLACK_MUSTANG_DEFENSE,
        &WADE_DEFENSE,
        &REVERSED_MEXICAN_DEFENSE,
        &HERRSTROM_GAMBIT,
        &KINGSIDE_FIANCHETTO,
        &QUIET_SYSTEM,
        &DRUNKEN_CAVALRY_VARIATION,
        &BASMAN_DEFENSE,
        &WARE_DEFENSE,
        &VOS_GAMBIT,
        &TENNISON_GAMBIT,
        &LISITSYN_GAMBIT,
        &DUTCH_VARIATION,
        &MYERS_POLISH_ATTACK,
        &PACHMAN_GAMBIT,
        &QUEENS_GAMBIT_INVITATION,
        &SANTASIERES_FOLLY,
        &OLD_INDIAN_ATTACK,
        &PIRC_INVITATION,
        &ARCTIC_DEFENSE,
        &SLAV_INVITATION,
    ],
    lines: &[
        Line {
            parent: &ZUKERTORT_OPENING,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<4>(),
            },
            moves: &[Normal {
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            }],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(2162623),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) {
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
            parent: &ZUKERTORT_OPENING,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<5>(),
            },
            moves: &[
                Normal {
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
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
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13834811764677541888),
                        white: Bitboard(2162623),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
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
        Line {
            parent: &ZUKERTORT_OPENING,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<6>(),
            },
            moves: &[
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
                    to: D5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(69524353607335680),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18444210833278894080),
                        white: Bitboard(2162623),
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
            parent: &ZUKERTORT_OPENING,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<5>(),
            },
            moves: &[
                Normal {
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
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
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(39582420959232),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13690700974648197120),
                        white: Bitboard(2424765),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
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
    ],
};
pub mod queenside_fianchetto_variation;
pub use queenside_fianchetto_variation::QUEENSIDE_FIANCHETTO_VARIATION;
pub mod polish_defense;
pub use polish_defense::POLISH_DEFENSE;
pub mod regina_nu_gambit;
pub use regina_nu_gambit::REGINA_NU_GAMBIT;
pub mod the_potato;
pub use the_potato::THE_POTATO;
pub mod reversed_grunfeld;
pub use reversed_grunfeld::REVERSED_GRUNFELD;
pub mod shabalov_gambit;
pub use shabalov_gambit::SHABALOV_GAMBIT;
pub mod speelsmet_gambit;
pub use speelsmet_gambit::SPEELSMET_GAMBIT;
pub mod the_walrus;
pub use the_walrus::THE_WALRUS;
pub mod st_george_defense;
pub use st_george_defense::ST_GEORGE_DEFENSE;
pub mod lisitsyn_gambit_deferred;
pub use lisitsyn_gambit_deferred::LISITSYN_GAMBIT_DEFERRED;
pub mod lemberger_gambit;
pub use lemberger_gambit::LEMBERGER_GAMBIT;
pub mod sicilian_invitation;
pub use sicilian_invitation::SICILIAN_INVITATION;
pub mod nimzo_larsen_variation;
pub use nimzo_larsen_variation::NIMZO_LARSEN_VARIATION;
pub mod ampel_variation;
pub use ampel_variation::AMPEL_VARIATION;
pub mod double_fianchetto_attack;
pub use double_fianchetto_attack::DOUBLE_FIANCHETTO_ATTACK;
pub mod ross_gambit;
pub use ross_gambit::ROSS_GAMBIT;
pub mod black_mustang_defense;
pub use black_mustang_defense::BLACK_MUSTANG_DEFENSE;
pub mod wade_defense;
pub use wade_defense::WADE_DEFENSE;
pub mod reversed_mexican_defense;
pub use reversed_mexican_defense::REVERSED_MEXICAN_DEFENSE;
pub mod herrstrom_gambit;
pub use herrstrom_gambit::HERRSTROM_GAMBIT;
pub mod kingside_fianchetto;
pub use kingside_fianchetto::KINGSIDE_FIANCHETTO;
pub mod quiet_system;
pub use quiet_system::QUIET_SYSTEM;
pub mod drunken_cavalry_variation;
pub use drunken_cavalry_variation::DRUNKEN_CAVALRY_VARIATION;
pub mod basman_defense;
pub use basman_defense::BASMAN_DEFENSE;
pub mod ware_defense;
pub use ware_defense::WARE_DEFENSE;
pub mod vos_gambit;
pub use vos_gambit::VOS_GAMBIT;
pub mod tennison_gambit;
pub use tennison_gambit::TENNISON_GAMBIT;
pub mod lisitsyn_gambit;
pub use lisitsyn_gambit::LISITSYN_GAMBIT;
pub mod dutch_variation;
pub use dutch_variation::DUTCH_VARIATION;
pub mod myers_polish_attack;
pub use myers_polish_attack::MYERS_POLISH_ATTACK;
pub mod pachman_gambit;
pub use pachman_gambit::PACHMAN_GAMBIT;
pub mod queens_gambit_invitation;
pub use queens_gambit_invitation::QUEENS_GAMBIT_INVITATION;
pub mod santasieres_folly;
pub use santasieres_folly::SANTASIERES_FOLLY;
pub mod old_indian_attack;
pub use old_indian_attack::OLD_INDIAN_ATTACK;
pub mod pirc_invitation;
pub use pirc_invitation::PIRC_INVITATION;
pub mod arctic_defense;
pub use arctic_defense::ARCTIC_DEFENSE;
pub mod slav_invitation;
pub use slav_invitation::SLAV_INVITATION;
