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
# use reco::book::QUEENS_GAMBIT_DECLINED;
assert_eq!(QUEENS_GAMBIT_DECLINED.original_name(), "Queen's Gambit Declined");
```"#
)]
pub static QUEENS_GAMBIT_DECLINED: Variation = Variation {
    name: "Queen's Gambit Declined",
    parent: None,
    variations: &[
        &ALAPIN_VARIATION,
        &ALBIN_COUNTERGAMBIT,
        &ALEKHINE_VARIATION,
        &ANTI_TARTAKOWER_VARIATION,
        &AUSTRIAN_ATTACK,
        &AUSTRIAN_DEFENSE,
        &BALTIC_DEFENSE,
        &BARMEN_VARIATION,
        &BEEN_KOOMEN_VARIATION,
        &CAMBRIDGE_SPRINGS_DEFENSE,
        &CAPABLANCA_VARIATION,
        &CHAROUSEK_VARIATION,
        &CHIGORIN_DEFENSE,
        &EXCHANGE_VARIATION,
        &HARRWITZ_ATTACK,
        &HASTINGS_VARIATION,
        &JANOWSKI_VARIATION,
        &KNIGHT_DEFENSE,
        &LASKER_DEFENSE,
        &MANHATTAN_VARIATION,
        &MARSHALL_DEFENSE,
        &MILES_VARIATION,
        &MODERN_VARIATION,
        &NEO_ORTHODOX_VARIATION,
        &NORMAL_DEFENSE,
        &ORTHODOX_DEFENSE,
        &PILLSBURY_ATTACK,
        &PSEUDO_TARRASCH_VARIATION,
        &QUEENS_KNIGHT_VARIATION,
        &RAGOZIN_DEFENSE,
        &ROCHLIN_VARIATION,
        &SEMI_SLAV,
        &SEMI_TARRASCH_DEFENSE,
        &SEMMERING_VARIATION,
        &SPIELMANN_VARIATION,
        &STONEWALL_VARIATION,
        &TARRASCH_DEFENSE,
        &TARTAKOWER_DEFENSE,
        &THREE_KNIGHTS,
        &THREE_KNIGHTS_VARIATION,
        &TRADITIONAL_VARIATION,
        &UHLMANN_VARIATION,
        &VIENNA_VARIATION,
        &WESTPHALIAN_VARIATION,
        &ZILBERMINTS_GAMBIT,
    ],
    lines: &[
        Line {
            parent: &QUEENS_GAMBIT_DECLINED,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<3>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346367333120),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18439724825837568000),
                        white: Bitboard(201389055),
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
            parent: &QUEENS_GAMBIT_DECLINED,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<5>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
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
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
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
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::G5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508046080),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(2594073660243312672),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13685089101659766784),
                        white: Bitboard(275082699705),
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
            parent: &QUEENS_GAMBIT_DECLINED,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<5>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::G5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346367333120),
                        knight: Bitboard(144150372448206912),
                        bishop: Bitboard(292734250656989216),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11526734582195945472),
                        white: Bitboard(275079558137),
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
                halfmoves: 4,
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
    ],
};
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod albin_countergambit;
pub use albin_countergambit::ALBIN_COUNTERGAMBIT;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod anti_tartakower_variation;
pub use anti_tartakower_variation::ANTI_TARTAKOWER_VARIATION;
pub mod austrian_attack;
pub use austrian_attack::AUSTRIAN_ATTACK;
pub mod austrian_defense;
pub use austrian_defense::AUSTRIAN_DEFENSE;
pub mod baltic_defense;
pub use baltic_defense::BALTIC_DEFENSE;
pub mod barmen_variation;
pub use barmen_variation::BARMEN_VARIATION;
pub mod been_koomen_variation;
pub use been_koomen_variation::BEEN_KOOMEN_VARIATION;
pub mod cambridge_springs_defense;
pub use cambridge_springs_defense::CAMBRIDGE_SPRINGS_DEFENSE;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod charousek_variation;
pub use charousek_variation::CHAROUSEK_VARIATION;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod harrwitz_attack;
pub use harrwitz_attack::HARRWITZ_ATTACK;
pub mod hastings_variation;
pub use hastings_variation::HASTINGS_VARIATION;
pub mod janowski_variation;
pub use janowski_variation::JANOWSKI_VARIATION;
pub mod knight_defense;
pub use knight_defense::KNIGHT_DEFENSE;
pub mod lasker_defense;
pub use lasker_defense::LASKER_DEFENSE;
pub mod manhattan_variation;
pub use manhattan_variation::MANHATTAN_VARIATION;
pub mod marshall_defense;
pub use marshall_defense::MARSHALL_DEFENSE;
pub mod miles_variation;
pub use miles_variation::MILES_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod neo_orthodox_variation;
pub use neo_orthodox_variation::NEO_ORTHODOX_VARIATION;
pub mod normal_defense;
pub use normal_defense::NORMAL_DEFENSE;
pub mod orthodox_defense;
pub use orthodox_defense::ORTHODOX_DEFENSE;
pub mod pillsbury_attack;
pub use pillsbury_attack::PILLSBURY_ATTACK;
pub mod pseudo_tarrasch_variation;
pub use pseudo_tarrasch_variation::PSEUDO_TARRASCH_VARIATION;
pub mod queens_knight_variation;
pub use queens_knight_variation::QUEENS_KNIGHT_VARIATION;
pub mod ragozin_defense;
pub use ragozin_defense::RAGOZIN_DEFENSE;
pub mod rochlin_variation;
pub use rochlin_variation::ROCHLIN_VARIATION;
pub mod semi_slav;
pub use semi_slav::SEMI_SLAV;
pub mod semi_tarrasch_defense;
pub use semi_tarrasch_defense::SEMI_TARRASCH_DEFENSE;
pub mod semmering_variation;
pub use semmering_variation::SEMMERING_VARIATION;
pub mod spielmann_variation;
pub use spielmann_variation::SPIELMANN_VARIATION;
pub mod stonewall_variation;
pub use stonewall_variation::STONEWALL_VARIATION;
pub mod tarrasch_defense;
pub use tarrasch_defense::TARRASCH_DEFENSE;
pub mod tartakower_defense;
pub use tartakower_defense::TARTAKOWER_DEFENSE;
pub mod three_knights;
pub use three_knights::THREE_KNIGHTS;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod uhlmann_variation;
pub use uhlmann_variation::UHLMANN_VARIATION;
pub mod vienna_variation;
pub use vienna_variation::VIENNA_VARIATION;
pub mod westphalian_variation;
pub use westphalian_variation::WESTPHALIAN_VARIATION;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
