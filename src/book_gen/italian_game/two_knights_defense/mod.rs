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
# use reco::book::italian_game::TWO_KNIGHTS_DEFENSE;
assert_eq!(TWO_KNIGHTS_DEFENSE.original_name(), "Italian Game: Two Knights Defense");
```"#
)]
pub static TWO_KNIGHTS_DEFENSE: Variation = Variation {
    name: "Two Knights Defense",
    parent: Some(&super::ITALIAN_GAME),
    variations: &[
        &BLACKBURNE_VARIATION,
        &COLMAN_VARIATION,
        &FEGATELLO_ATTACK,
        &FRIED_LIVER_ATTACK,
        &FRITZ,
        &FRITZ_VARIATION,
        &KEIDANSKY_VARIATION,
        &KLOSS_GAMBIT,
        &KNIGHT_ATTACK,
        &KNORRE_VARIATION,
        &LOLLI_ATTACK,
        &MAROCZY_VARIATION,
        &MAX_LANGE_ATTACK,
        &MODERN_BISHOPS_OPENING,
        &OPEN_VARIATION,
        &PAOLI_VARIATION,
        &PERREUX_VARIATION,
        &PINCUS_VARIATION,
        &POLERIO_DEFENSE,
        &PONZIANI_STEINITZ_GAMBIT,
        &STEINITZ_VARIATION,
        &TRAXLER_COUNTERATTACK,
        &TRAXLER_VARIATION,
        &ULVESTAD_VARIATION,
        &YURDANSKY_ATTACK,
    ],
    lines: &[
        Line {
            parent: &TWO_KNIGHTS_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<55>(),
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
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(39582420697090),
                        bishop: Bitboard(2594073385432514564),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13686197443740303360),
                        white: Bitboard(337702815),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
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
            parent: &TWO_KNIGHTS_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<55>(),
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Bishop,
                    from: F8,
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
                    role: Bishop,
                    from: C5,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: Some(Bishop),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: C6,
                    capture: Some(Knight),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(65029584701155072),
                        knight: Bitboard(35184506306562),
                        bishop: Bitboard(288230651096727552),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(11378107032893652992),
                        white: Bitboard(275213510507),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
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
            parent: &TWO_KNIGHTS_DEFENSE,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<58>(),
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: None,
                    to: G5,
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
                    role: Pawn,
                    from: E4,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: C6,
                    capture: None,
                    to: A5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C4,
                    capture: None,
                    to: B5,
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
                    from: D5,
                    capture: Some(Pawn),
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: Some(Pawn),
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: B5,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63336336525946624),
                        knight: Bitboard(35463544963074),
                        bishop: Bitboard(2594073385365409796),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13682256798361321472),
                        white: Bitboard(274877972383),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
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
pub mod blackburne_variation;
pub use blackburne_variation::BLACKBURNE_VARIATION;
pub mod colman_variation;
pub use colman_variation::COLMAN_VARIATION;
pub mod fegatello_attack;
pub use fegatello_attack::FEGATELLO_ATTACK;
pub mod fried_liver_attack;
pub use fried_liver_attack::FRIED_LIVER_ATTACK;
pub mod fritz;
pub use fritz::FRITZ;
pub mod fritz_variation;
pub use fritz_variation::FRITZ_VARIATION;
pub mod keidansky_variation;
pub use keidansky_variation::KEIDANSKY_VARIATION;
pub mod kloss_gambit;
pub use kloss_gambit::KLOSS_GAMBIT;
pub mod knight_attack;
pub use knight_attack::KNIGHT_ATTACK;
pub mod knorre_variation;
pub use knorre_variation::KNORRE_VARIATION;
pub mod lolli_attack;
pub use lolli_attack::LOLLI_ATTACK;
pub mod maroczy_variation;
pub use maroczy_variation::MAROCZY_VARIATION;
pub mod max_lange_attack;
pub use max_lange_attack::MAX_LANGE_ATTACK;
pub mod modern_bishops_opening;
pub use modern_bishops_opening::MODERN_BISHOPS_OPENING;
pub mod open_variation;
pub use open_variation::OPEN_VARIATION;
pub mod paoli_variation;
pub use paoli_variation::PAOLI_VARIATION;
pub mod perreux_variation;
pub use perreux_variation::PERREUX_VARIATION;
pub mod pincus_variation;
pub use pincus_variation::PINCUS_VARIATION;
pub mod polerio_defense;
pub use polerio_defense::POLERIO_DEFENSE;
pub mod ponziani_steinitz_gambit;
pub use ponziani_steinitz_gambit::PONZIANI_STEINITZ_GAMBIT;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod traxler_counterattack;
pub use traxler_counterattack::TRAXLER_COUNTERATTACK;
pub mod traxler_variation;
pub use traxler_variation::TRAXLER_VARIATION;
pub mod ulvestad_variation;
pub use ulvestad_variation::ULVESTAD_VARIATION;
pub mod yurdansky_attack;
pub use yurdansky_attack::YURDANSKY_ATTACK;
