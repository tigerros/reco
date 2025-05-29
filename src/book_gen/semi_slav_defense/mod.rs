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
# use reco::book::SEMI_SLAV_DEFENSE;
assert_eq!(SEMI_SLAV_DEFENSE.original_name(), "Semi-Slav Defense");
```"#
)]
pub static SEMI_SLAV_DEFENSE: Variation = Variation {
    name: "Semi-Slav Defense",
    parent: None,
    variations: &[
        &ACCELERATED_MOVE_ORDER,
        &ACCELERATED_MERAN_VARIATION,
        &ANTI_NOTEBOOM,
        &MARSHALL_GAMBIT,
        &GUNDERAM_GAMBIT,
        &NOTEBOOM_VARIATION,
        &BOGOLJUBOW_VARIATION,
        &SEMI_MERAN_VARIATION,
        &ANTI_MOSCOW_GAMBIT,
        &MAIN_LINE,
        &MERAN_VARIATION,
        &NORMAL_VARIATION,
        &BOTVINNIK_VARIATION,
        &QUIET_VARIATION,
        &STONEWALL_DEFENSE,
        &CHIGORIN_DEFENSE,
        &RUBINSTEIN_SYSTEM,
        &ROMIH_VARIATION,
        &STOLTZ_VARIATION,
    ],
    lines: &[Line {
        parent: &SEMI_SLAV_DEFENSE,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<43>(),
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
                role: Pawn,
                from: D7,
                capture: None,
                to: D5,
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
                from: C7,
                capture: None,
                to: C6,
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
            ),
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
pub mod accelerated_move_order;
pub use accelerated_move_order::ACCELERATED_MOVE_ORDER;
pub mod accelerated_meran_variation;
pub use accelerated_meran_variation::ACCELERATED_MERAN_VARIATION;
pub mod anti_noteboom;
pub use anti_noteboom::ANTI_NOTEBOOM;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod gunderam_gambit;
pub use gunderam_gambit::GUNDERAM_GAMBIT;
pub mod noteboom_variation;
pub use noteboom_variation::NOTEBOOM_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod semi_meran_variation;
pub use semi_meran_variation::SEMI_MERAN_VARIATION;
pub mod anti_moscow_gambit;
pub use anti_moscow_gambit::ANTI_MOSCOW_GAMBIT;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod meran_variation;
pub use meran_variation::MERAN_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod quiet_variation;
pub use quiet_variation::QUIET_VARIATION;
pub mod stonewall_defense;
pub use stonewall_defense::STONEWALL_DEFENSE;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod rubinstein_system;
pub use rubinstein_system::RUBINSTEIN_SYSTEM;
pub mod romih_variation;
pub use romih_variation::ROMIH_VARIATION;
pub mod stoltz_variation;
pub use stoltz_variation::STOLTZ_VARIATION;
