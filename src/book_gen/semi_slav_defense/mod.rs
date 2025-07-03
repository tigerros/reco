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
use core::unreachable;
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
# use reco::book::SEMI_SLAV_DEFENSE;
assert_eq!(SEMI_SLAV_DEFENSE.original_name(), "Semi-Slav Defense");
```"#
)]
pub static SEMI_SLAV_DEFENSE: Variation = Variation {
    name: "Semi-Slav Defense",
    parent: None,
    variations: &[
        &ACCELERATED_MERAN_VARIATION,
        &ACCELERATED_MOVE_ORDER,
        &ANTI_MOSCOW_GAMBIT,
        &ANTI_NOTEBOOM,
        &BOGOLJUBOW_VARIATION,
        &BOTVINNIK_VARIATION,
        &CHIGORIN_DEFENSE,
        &GUNDERAM_GAMBIT,
        &MAIN_LINE,
        &MARSHALL_GAMBIT,
        &MERAN_VARIATION,
        &NORMAL_VARIATION,
        &NOTEBOOM_VARIATION,
        &QUIET_VARIATION,
        &ROMIH_VARIATION,
        &RUBINSTEIN_SYSTEM,
        &SEMI_MERAN_VARIATION,
        &STOLTZ_VARIATION,
        &STONEWALL_DEFENSE,
    ],
    lines: &[Line {
        parent: &SEMI_SLAV_DEFENSE,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<4>()),
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
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(63916844507001600),
                    knight: Bitboard(144150372450304000),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13826952489921937408),
                    white: Bitboard(203748285),
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
    }],
};
pub mod accelerated_meran_variation;
pub use accelerated_meran_variation::ACCELERATED_MERAN_VARIATION;
pub mod accelerated_move_order;
pub use accelerated_move_order::ACCELERATED_MOVE_ORDER;
pub mod anti_moscow_gambit;
pub use anti_moscow_gambit::ANTI_MOSCOW_GAMBIT;
pub mod anti_noteboom;
pub use anti_noteboom::ANTI_NOTEBOOM;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod gunderam_gambit;
pub use gunderam_gambit::GUNDERAM_GAMBIT;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod meran_variation;
pub use meran_variation::MERAN_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod noteboom_variation;
pub use noteboom_variation::NOTEBOOM_VARIATION;
pub mod quiet_variation;
pub use quiet_variation::QUIET_VARIATION;
pub mod romih_variation;
pub use romih_variation::ROMIH_VARIATION;
pub mod rubinstein_system;
pub use rubinstein_system::RUBINSTEIN_SYSTEM;
pub mod semi_meran_variation;
pub use semi_meran_variation::SEMI_MERAN_VARIATION;
pub mod stoltz_variation;
pub use stoltz_variation::STOLTZ_VARIATION;
pub mod stonewall_defense;
pub use stonewall_defense::STONEWALL_DEFENSE;
