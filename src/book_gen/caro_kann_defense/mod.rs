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
# use reco::book::CARO_KANN_DEFENSE;
assert_eq!(CARO_KANN_DEFENSE.original_name(), "Caro-Kann Defense");
```"#
)]
pub static CARO_KANN_DEFENSE: Variation = Variation {
    name: "Caro-Kann Defense",
    parent: None,
    variations: &[
        &TARTAKOWER_VARIATION,
        &TWO_KNIGHTS_ATTACK,
        &CAMPOMANES_ATTACK,
        &KARPOV_VARIATION,
        &MASSACHUSETTS_DEFENSE,
        &ACCELERATED_PANOV_ATTACK,
        &LABAHN_ATTACK,
        &PANOV_BOTVINNIK,
        &MAIN_LINE,
        &GURGENIDZE_SYSTEM,
        &MASI_VARIATION,
        &DE_BRUYCKER_DEFENSE,
        &EXCHANGE_VARIATION,
        &MIESES_ATTACK,
        &PANOV_ATTACK,
        &SCORPION_HORUS_GAMBIT,
        &RASA_STUDIER_GAMBIT,
        &FINNISH_VARIATION,
        &CLASSICAL_VARIATION,
        &ENDGAME_OFFER,
        &ADVANCE,
        &EUWE_ATTACK,
        &APOCALYPSE_ATTACK,
        &MAROCZY_VARIATION,
        &MODERN_VARIATION,
        &GURGENIDZE_COUNTERATTACK,
        &SPIKE_VARIATION,
        &ENDGAME_VARIATION,
        &EDINBURGH_VARIATION,
        &HILLBILLY_ATTACK,
        &MIESES_GAMBIT,
        &HECTOR_GAMBIT,
        &ULYSSES_GAMBIT,
        &VON_HENNIG_GAMBIT,
        &ALIEN_GAMBIT,
        &ST_PATRICKS_ATTACK,
        &BRONSTEIN_LARSEN_VARIATION,
        &GOLDMAN_VARIATION,
        &ALEKHINE_GAMBIT,
        &MARTIAN_GAMBIT,
        &ADVANCE_VARIATION,
        &BREYER_VARIATION,
        &TOIKKANEN_GAMBIT,
        &FORGACS_VARIATION,
    ],
    lines: &[
        Line {
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<10>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70654617469382400),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18445341096872509440),
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<12>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70654617603598080),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18445341096872509440),
                        white: Bitboard(402712575),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<10>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68402852015435520),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18443089331418562560),
                        white: Bitboard(268759037),
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<15>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68402817789912832),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18443089297327259648),
                        white: Bitboard(134539261),
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<12>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
                        pawn: Bitboard(68402852149651200),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18443089331418562560),
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<15>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68402852149651200),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18443089331418562560),
                        white: Bitboard(402974717),
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
            parent: &CARO_KANN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<10>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70654617469382400),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18445341096872509440),
                        white: Bitboard(268759037),
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
pub mod tartakower_variation;
pub use tartakower_variation::TARTAKOWER_VARIATION;
pub mod two_knights_attack;
pub use two_knights_attack::TWO_KNIGHTS_ATTACK;
pub mod campomanes_attack;
pub use campomanes_attack::CAMPOMANES_ATTACK;
pub mod karpov_variation;
pub use karpov_variation::KARPOV_VARIATION;
pub mod massachusetts_defense;
pub use massachusetts_defense::MASSACHUSETTS_DEFENSE;
pub mod accelerated_panov_attack;
pub use accelerated_panov_attack::ACCELERATED_PANOV_ATTACK;
pub mod labahn_attack;
pub use labahn_attack::LABAHN_ATTACK;
pub mod panov_botvinnik;
pub use panov_botvinnik::PANOV_BOTVINNIK;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod gurgenidze_system;
pub use gurgenidze_system::GURGENIDZE_SYSTEM;
pub mod masi_variation;
pub use masi_variation::MASI_VARIATION;
pub mod de_bruycker_defense;
pub use de_bruycker_defense::DE_BRUYCKER_DEFENSE;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod mieses_attack;
pub use mieses_attack::MIESES_ATTACK;
pub mod panov_attack;
pub use panov_attack::PANOV_ATTACK;
pub mod scorpion_horus_gambit;
pub use scorpion_horus_gambit::SCORPION_HORUS_GAMBIT;
pub mod rasa_studier_gambit;
pub use rasa_studier_gambit::RASA_STUDIER_GAMBIT;
pub mod finnish_variation;
pub use finnish_variation::FINNISH_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod endgame_offer;
pub use endgame_offer::ENDGAME_OFFER;
pub mod advance;
pub use advance::ADVANCE;
pub mod euwe_attack;
pub use euwe_attack::EUWE_ATTACK;
pub mod apocalypse_attack;
pub use apocalypse_attack::APOCALYPSE_ATTACK;
pub mod maroczy_variation;
pub use maroczy_variation::MAROCZY_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod gurgenidze_counterattack;
pub use gurgenidze_counterattack::GURGENIDZE_COUNTERATTACK;
pub mod spike_variation;
pub use spike_variation::SPIKE_VARIATION;
pub mod endgame_variation;
pub use endgame_variation::ENDGAME_VARIATION;
pub mod edinburgh_variation;
pub use edinburgh_variation::EDINBURGH_VARIATION;
pub mod hillbilly_attack;
pub use hillbilly_attack::HILLBILLY_ATTACK;
pub mod mieses_gambit;
pub use mieses_gambit::MIESES_GAMBIT;
pub mod hector_gambit;
pub use hector_gambit::HECTOR_GAMBIT;
pub mod ulysses_gambit;
pub use ulysses_gambit::ULYSSES_GAMBIT;
pub mod von_hennig_gambit;
pub use von_hennig_gambit::VON_HENNIG_GAMBIT;
pub mod alien_gambit;
pub use alien_gambit::ALIEN_GAMBIT;
pub mod st_patricks_attack;
pub use st_patricks_attack::ST_PATRICKS_ATTACK;
pub mod bronstein_larsen_variation;
pub use bronstein_larsen_variation::BRONSTEIN_LARSEN_VARIATION;
pub mod goldman_variation;
pub use goldman_variation::GOLDMAN_VARIATION;
pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;
pub mod martian_gambit;
pub use martian_gambit::MARTIAN_GAMBIT;
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod toikkanen_gambit;
pub use toikkanen_gambit::TOIKKANEN_GAMBIT;
pub mod forgacs_variation;
pub use forgacs_variation::FORGACS_VARIATION;
